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
    cat >&2 <<'EOF'
usage: ./scripts/verify.sh [--quick|--deny-only|--help]

  no argument   run quick checks, rustdoc, Markdown links, and cargo-deny
  --quick       run format, clippy, tests, and Markdown links
  --deny-only   run dependency policy only
  --help        show this help
EOF
}

require_tool() {
    local tool="$1"
    if ! command -v "$tool" >/dev/null 2>&1; then
        echo "required tool not found: $tool" >&2
        exit 1
    fi
}

ensure_cargo_deny() {
    local expected="cargo-deny $CARGO_DENY_VERSION"
    local actual=""

    if command -v cargo-deny >/dev/null 2>&1; then
        actual="$(cargo-deny --version)"
    fi

    if [[ "$actual" != "$expected" ]]; then
        echo "installing pinned cargo-deny $CARGO_DENY_VERSION into $CARGO_HOME" >&2
        "${CARGO[@]}" install cargo-deny --version "$CARGO_DENY_VERSION" --locked --force
    fi
}

run_markdown_links() {
    require_tool python3
    python3 scripts/check-markdown-links.py
}

run_dependency_policy() {
    ensure_cargo_deny
    "${CARGO[@]}" deny check
}

run_quick() {
    "${CARGO[@]}" fmt --all -- --check
    "${CARGO[@]}" clippy --workspace --all-targets --locked -- -D warnings
    "${CARGO[@]}" test --workspace --all-targets --locked
    run_markdown_links
}

mode="${1:-all}"

if [[ "$mode" == "--help" || "$mode" == "-h" ]]; then
    usage
    exit 0
fi

require_tool rustup
echo "verification mode: $mode" >&2
echo "Rust toolchain: $TOOLCHAIN" >&2

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
