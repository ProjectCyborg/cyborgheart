#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

TOOLCHAIN="${CYBORGHEART_RUST_TOOLCHAIN:-1.97.0}"
CARGO_DENY_VERSION="${CYBORGHEART_CARGO_DENY_VERSION:-0.20.2}"
CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
export PATH="$CARGO_HOME/bin:$PATH"
CARGO=(rustup run "$TOOLCHAIN" cargo)

usage() {
    echo "usage: $0 [--quick|--deny-only]" >&2
}

ensure_cargo_deny() {
    local expected="cargo-deny $CARGO_DENY_VERSION"
    local actual=""

    if command -v cargo-deny >/dev/null 2>&1; then
        actual="$(cargo-deny --version)"
    fi

    if [[ "$actual" != "$expected" ]]; then
        "${CARGO[@]}" install cargo-deny --version "$CARGO_DENY_VERSION" --locked --force
    fi
}

run_dependency_policy() {
    ensure_cargo_deny
    "${CARGO[@]}" deny check
}

run_quick() {
    "${CARGO[@]}" fmt --all -- --check
    "${CARGO[@]}" clippy --workspace --all-targets --locked -- -D warnings
    "${CARGO[@]}" test --workspace --all-targets --locked
}

mode="${1:-all}"

case "$mode" in
    all)
        run_quick
        RUSTDOCFLAGS="-D warnings" "${CARGO[@]}" doc --workspace --no-deps --locked
        run_dependency_policy
        ;;
    --quick)
        run_quick
        ;;
    --deny-only)
        run_dependency_policy
        ;;
    *)
        usage
        exit 2
        ;;
esac
