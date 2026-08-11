#!/usr/bin/env bash
# Git タグと Cargo.toml の version が一致するか検証する
#
# 使い方:
#   ./scripts/check_release_version.sh [tag]
#
# タグの取得優先順位:
#   1. 第1引数
#   2. 環境変数 CHECK_RELEASE_TAG
#   3. 環境変数 GITHUB_REF_NAME
#
# 終了コード:
#   0: タグが {tag_prefix}{Cargo.toml version} と一致
#   非0: 不一致・空・不正形式・設定/ファイル不備
#
# 上書き用環境変数:
#   CI_CONFIG_PATH   - config/ci.toml のパス
#   CARGO_TOML_PATH  - Cargo.toml のパス

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=lib/ci_config.sh
source "${SCRIPT_DIR}/lib/ci_config.sh"

REPO_ROOT="$(ci_config_repo_root)"

read_cargo_version() {
  local cargo_toml="${CARGO_TOML_PATH:-}"
  if [ -z "${cargo_toml}" ]; then
    local relative
    relative="$(ci_config_get paths cargo_toml 2>/dev/null || echo "Cargo.toml")"
    cargo_toml="${REPO_ROOT}/${relative}"
  fi

  if [ ! -f "${cargo_toml}" ]; then
    echo "error: Cargo.toml not found: ${cargo_toml}" >&2
    return 1
  fi

  local version
  version="$(sed -n 's/^version = "\([^"]*\)"/\1/p' "${cargo_toml}" | head -n1)"
  if [ -z "${version}" ]; then
    echo "error: could not read package version from ${cargo_toml}" >&2
    return 1
  fi
  printf '%s\n' "${version}"
}

resolve_tag() {
  if [ "${#}" -ge 1 ] && [ -n "${1}" ]; then
    printf '%s\n' "${1}"
    return 0
  fi
  if [ -n "${CHECK_RELEASE_TAG:-}" ]; then
    printf '%s\n' "${CHECK_RELEASE_TAG}"
    return 0
  fi
  if [ -n "${GITHUB_REF_NAME:-}" ]; then
    printf '%s\n' "${GITHUB_REF_NAME}"
    return 0
  fi
  echo "error: release tag is empty; pass as argument or set CHECK_RELEASE_TAG / GITHUB_REF_NAME" >&2
  return 1
}

is_valid_semver_like() {
  local version="$1"
  # 簡易: MAJOR.MINOR.PATCH（追加ラベルなしのセマンティック版）
  [[ "${version}" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]
}

main() {
  local tag crate_version tag_prefix expected

  tag="$(resolve_tag "${@:-}")" || exit 1
  if [ -z "${tag}" ]; then
    echo "error: release tag is empty" >&2
    exit 1
  fi

  tag_prefix="$(ci_config_get release tag_prefix)" || exit 1
  crate_version="$(read_cargo_version)" || exit 1

  if ! is_valid_semver_like "${crate_version}"; then
    echo "error: invalid Cargo.toml version format: '${crate_version}' (expected MAJOR.MINOR.PATCH)" >&2
    exit 1
  fi

  expected="${tag_prefix}${crate_version}"

  if [[ ! "${tag}" =~ ^${tag_prefix} ]]; then
    echo "error: tag '${tag}' does not start with configured prefix '${tag_prefix}'" >&2
    echo "  Cargo.toml version: ${crate_version}" >&2
    echo "  expected tag:       ${expected}" >&2
    exit 1
  fi

  local tag_version="${tag#"${tag_prefix}"}"
  if [ -z "${tag_version}" ]; then
    echo "error: tag '${tag}' has empty version after prefix '${tag_prefix}'" >&2
    exit 1
  fi

  if ! is_valid_semver_like "${tag_version}"; then
    echo "error: invalid tag version format: '${tag}' (expected ${tag_prefix}MAJOR.MINOR.PATCH)" >&2
    echo "  Cargo.toml version: ${crate_version}" >&2
    exit 1
  fi

  if [ "${tag}" != "${expected}" ]; then
    echo "error: git tag does not match Cargo.toml version" >&2
    echo "  tag:                ${tag}" >&2
    echo "  Cargo.toml version: ${crate_version}" >&2
    echo "  expected tag:       ${expected}" >&2
    exit 1
  fi

  echo "OK: tag '${tag}' matches Cargo.toml version '${crate_version}' (prefix='${tag_prefix}')"
}

main "$@"
