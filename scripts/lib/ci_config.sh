#!/usr/bin/env bash
# config/ci.toml からキーを読み取るヘルパー
# 使い方: ci_config_get <section> <key>
# 環境変数 CI_CONFIG_PATH で設定ファイルパスを上書き可能

set -euo pipefail

ci_config_repo_root() {
  local script_dir
  script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  # scripts/lib -> リポジトリルート
  cd "${script_dir}/../.." && pwd
}

ci_config_path() {
  if [ -n "${CI_CONFIG_PATH:-}" ]; then
    echo "${CI_CONFIG_PATH}"
    return 0
  fi
  echo "$(ci_config_repo_root)/config/ci.toml"
}

# TOML の単純な string キーを読む（ネスト・配列・インラインテーブル非対応）
# 引用符付き値のみ想定: key = "value"
ci_config_get() {
  local section="$1"
  local key="$2"
  local config_file
  config_file="$(ci_config_path)"

  if [ ! -f "${config_file}" ]; then
    echo "error: CI config not found: ${config_file}" >&2
    return 1
  fi

  local in_section=0
  local line stripped value

  while IFS= read -r line || [ -n "${line}" ]; do
    # コメントと空行を除去
    stripped="${line%%#*}"
    stripped="$(echo "${stripped}" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')"
    if [ -z "${stripped}" ]; then
      continue
    fi

    if [[ "${stripped}" =~ ^\[([a-zA-Z0-9_]+)\]$ ]]; then
      if [ "${BASH_REMATCH[1]}" = "${section}" ]; then
        in_section=1
      else
        in_section=0
      fi
      continue
    fi

    if [ "${in_section}" -eq 1 ] && [[ "${stripped}" =~ ^${key}[[:space:]]*=[[:space:]]*\"([^\"]*)\"[[:space:]]*$ ]]; then
      value="${BASH_REMATCH[1]}"
      printf '%s\n' "${value}"
      return 0
    fi
  done < "${config_file}"

  echo "error: key [${section}].${key} not found in ${config_file}" >&2
  return 1
}
