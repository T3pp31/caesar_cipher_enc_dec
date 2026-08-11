#!/usr/bin/env bash
# check_release_version.sh / ci_config.sh のシェル統合テスト
#
# ## テスト観点表（等価分割・境界値）
#
# | テスト観点 | 分類 | 入力例 | 期待値 |
# |---|---|---|---|
# | タグ = prefix + Cargo version | 正常系 | v1.0.10 / 1.0.10 | exit 0 |
# | CHECK_RELEASE_TAG | 正常系 | env のみ | exit 0 |
# | GITHUB_REF_NAME | 正常系 | env のみ | exit 0 |
# | カスタム prefix | 正常系 | rel-2.0.0 | exit 0 |
# | タグ不一致 | 異常系 | v1.0.9 vs 1.0.10 | exit ≠0 |
# | タグ空 | 異常系 | 引数・env なし | exit ≠0 |
# | prefix 不一致 | 異常系 | 1.0.10（vなし） | exit ≠0 |
# | 不正 version | 異常系 | 1.0 | exit ≠0 |
# | 不正 tag | 異常系 | v1.0.10-beta | exit ≠0 |
# | Cargo.toml 欠落 | 異常系 | 欠落パス | exit ≠0 |
# | 設定キー欠落 | 異常系 | tag_prefix なし | exit ≠0 |
# | prefix のみ | 境界値 | tag=v | exit ≠0 |
#
# 実行:
#   bash tests/check_release_version_tests.sh
#
# カバレッジ:
#   シェル分岐は本スイートで網羅。Rust 側は `cargo test` / `cargo llvm-cov` を利用。

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
CHECK_SCRIPT="${REPO_ROOT}/scripts/check_release_version.sh"
CI_CONFIG_LIB="${REPO_ROOT}/scripts/lib/ci_config.sh"

PASS=0
FAIL=0

assert_exit() {
  local name="$1"
  local expected_ok="$2" # "ok" or "ng"
  shift 2
  local status=0
  set +e
  "$@" >/tmp/check_release_version_test_out.txt 2>/tmp/check_release_version_test_err.txt
  status=$?
  set -e

  if [ "${expected_ok}" = "ok" ]; then
    if [ "${status}" -eq 0 ]; then
      echo "PASS: ${name}"
      PASS=$((PASS + 1))
    else
      echo "FAIL: ${name} (expected exit 0, got ${status})"
      cat /tmp/check_release_version_test_err.txt >&2 || true
      FAIL=$((FAIL + 1))
    fi
  else
    if [ "${status}" -ne 0 ]; then
      echo "PASS: ${name}"
      PASS=$((PASS + 1))
    else
      echo "FAIL: ${name} (expected non-zero exit, got 0)"
      cat /tmp/check_release_version_test_out.txt >&2 || true
      FAIL=$((FAIL + 1))
    fi
  fi
}

assert_output_contains() {
  local name="$1"
  local needle="$2"
  if grep -q "${needle}" /tmp/check_release_version_test_err.txt \
    || grep -q "${needle}" /tmp/check_release_version_test_out.txt; then
    echo "PASS: ${name}"
    PASS=$((PASS + 1))
  else
    echo "FAIL: ${name} (expected output to contain '${needle}')"
    FAIL=$((FAIL + 1))
  fi
}

make_fixture() {
  local dir="$1"
  local version="$2"
  local prefix="$3"
  mkdir -p "${dir}"
  cat > "${dir}/Cargo.toml" <<EOF
[package]
name = "fixture_crate"
version = "${version}"
edition = "2021"
EOF
  cat > "${dir}/ci.toml" <<EOF
[release]
tag_prefix = "${prefix}"

[crates_io]
user_agent = "fixture-ua"

[paths]
cargo_toml = "Cargo.toml"
EOF
}

run_check() {
  local fixture="$1"
  shift
  env -u CHECK_RELEASE_TAG -u GITHUB_REF_NAME \
    CI_CONFIG_PATH="${fixture}/ci.toml" \
    CARGO_TOML_PATH="${fixture}/Cargo.toml" \
    bash "${CHECK_SCRIPT}" "$@"
}

TMP_ROOT="$(mktemp -d)"
trap 'rm -rf "${TMP_ROOT}"' EXIT

# -----------------------------------------------------------------------------
# 正常系
# -----------------------------------------------------------------------------

# Given: version 1.0.10 / prefix v
# When: tag v1.0.10
# Then: exit 0
FIX="${TMP_ROOT}/ok_match"
make_fixture "${FIX}" "1.0.10" "v"
assert_exit "normal: matching tag argument" ok run_check "${FIX}" "v1.0.10"

# Given: CHECK_RELEASE_TAG=v1.0.10
# When: 引数なし
# Then: exit 0
FIX="${TMP_ROOT}/ok_env_check"
make_fixture "${FIX}" "1.0.10" "v"
assert_exit "normal: CHECK_RELEASE_TAG" ok \
  env -u GITHUB_REF_NAME \
    CI_CONFIG_PATH="${FIX}/ci.toml" \
    CARGO_TOML_PATH="${FIX}/Cargo.toml" \
    CHECK_RELEASE_TAG="v1.0.10" \
    bash "${CHECK_SCRIPT}"

# Given: GITHUB_REF_NAME=v2.3.4
# When: 引数なし
# Then: exit 0
FIX="${TMP_ROOT}/ok_env_github"
make_fixture "${FIX}" "2.3.4" "v"
assert_exit "normal: GITHUB_REF_NAME" ok \
  env -u CHECK_RELEASE_TAG \
    CI_CONFIG_PATH="${FIX}/ci.toml" \
    CARGO_TOML_PATH="${FIX}/Cargo.toml" \
    GITHUB_REF_NAME="v2.3.4" \
    bash "${CHECK_SCRIPT}"

# Given: カスタム prefix rel-
# When: tag rel-2.0.0
# Then: exit 0
FIX="${TMP_ROOT}/ok_custom_prefix"
make_fixture "${FIX}" "2.0.0" "rel-"
assert_exit "normal: custom tag prefix" ok run_check "${FIX}" "rel-2.0.0"

# Given: 実リポジトリ設定
# When: 実 Cargo.toml の version でタグを組み立て
# Then: exit 0
REAL_VERSION="$(sed -n 's/^version = "\([^"]*\)"/\1/p' "${REPO_ROOT}/Cargo.toml" | head -n1)"
assert_exit "normal: real repo config" ok \
  bash "${CHECK_SCRIPT}" "v${REAL_VERSION}"

# -----------------------------------------------------------------------------
# 異常系（正常系以上）
# -----------------------------------------------------------------------------

# Given: 不一致タグ
# When: v1.0.9 vs 1.0.10
# Then: exit ≠0 かつメッセージ
FIX="${TMP_ROOT}/ng_mismatch"
make_fixture "${FIX}" "1.0.10" "v"
assert_exit "error: version mismatch" ng run_check "${FIX}" "v1.0.9"
assert_output_contains "error: mismatch message" "does not match"

# Given: タグ未指定
# When: 引数・env なし
# Then: exit ≠0
FIX="${TMP_ROOT}/ng_empty"
make_fixture "${FIX}" "1.0.10" "v"
assert_exit "error: empty tag" ng \
  env -u CHECK_RELEASE_TAG -u GITHUB_REF_NAME \
    CI_CONFIG_PATH="${FIX}/ci.toml" \
    CARGO_TOML_PATH="${FIX}/Cargo.toml" \
    bash "${CHECK_SCRIPT}"

# Given: prefix なしタグ
# When: 1.0.10
# Then: exit ≠0
FIX="${TMP_ROOT}/ng_no_prefix"
make_fixture "${FIX}" "1.0.10" "v"
assert_exit "error: missing prefix" ng run_check "${FIX}" "1.0.10"
assert_output_contains "error: prefix message" "does not start with"

# Given: 不正 Cargo version
# When: version=1.0
# Then: exit ≠0
FIX="${TMP_ROOT}/ng_bad_cargo_ver"
make_fixture "${FIX}" "1.0" "v"
assert_exit "error: invalid cargo version" ng run_check "${FIX}" "v1.0"
assert_output_contains "error: invalid cargo version message" "invalid Cargo.toml version"

# Given: 不正タグ形式
# When: v1.0.10-beta
# Then: exit ≠0
FIX="${TMP_ROOT}/ng_bad_tag"
make_fixture "${FIX}" "1.0.10" "v"
assert_exit "error: invalid tag format" ng run_check "${FIX}" "v1.0.10-beta"
assert_output_contains "error: invalid tag message" "invalid tag version format"

# Given: Cargo.toml 欠落
# When: 存在しないパス
# Then: exit ≠0
FIX="${TMP_ROOT}/ng_missing_cargo"
make_fixture "${FIX}" "1.0.10" "v"
rm -f "${FIX}/Cargo.toml"
assert_exit "error: missing Cargo.toml" ng run_check "${FIX}" "v1.0.10"

# Given: tag_prefix 欠落の設定
# When: 検証実行
# Then: exit ≠0
FIX="${TMP_ROOT}/ng_missing_key"
mkdir -p "${FIX}"
cat > "${FIX}/Cargo.toml" <<'EOF'
[package]
name = "fixture_crate"
version = "1.0.10"
edition = "2021"
EOF
cat > "${FIX}/ci.toml" <<'EOF'
[crates_io]
user_agent = "fixture-ua"
EOF
assert_exit "error: missing tag_prefix key" ng run_check "${FIX}" "v1.0.10"

# Given: prefix のみのタグ
# When: tag=v
# Then: exit ≠0
FIX="${TMP_ROOT}/ng_prefix_only"
make_fixture "${FIX}" "1.0.10" "v"
assert_exit "boundary: prefix-only tag" ng run_check "${FIX}" "v"

# -----------------------------------------------------------------------------
# ci_config_get 単体
# -----------------------------------------------------------------------------

# shellcheck source=../scripts/lib/ci_config.sh
source "${CI_CONFIG_LIB}"

# Given: 実 config/ci.toml
# When: crates_io.user_agent を読む
# Then: 期待文字列
UA="$(CI_CONFIG_PATH="${REPO_ROOT}/config/ci.toml" ci_config_get crates_io user_agent)"
if [ "${UA}" = "caesar_cipher_enc_dec-release (github-actions)" ]; then
  echo "PASS: ci_config_get user_agent"
  PASS=$((PASS + 1))
else
  echo "FAIL: ci_config_get user_agent (got '${UA}')"
  FAIL=$((FAIL + 1))
fi

# Given: 存在しないキー
# When: ci_config_get
# Then: 失敗
set +e
CI_CONFIG_PATH="${REPO_ROOT}/config/ci.toml" ci_config_get release missing_key >/dev/null 2>&1
status=$?
set -e
if [ "${status}" -ne 0 ]; then
  echo "PASS: ci_config_get missing key"
  PASS=$((PASS + 1))
else
  echo "FAIL: ci_config_get missing key"
  FAIL=$((FAIL + 1))
fi

echo ""
echo "Results: ${PASS} passed, ${FAIL} failed"
if [ "${FAIL}" -ne 0 ]; then
  exit 1
fi
