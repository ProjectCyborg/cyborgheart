#!/usr/bin/env python3
"""Validate repository-local Markdown file links without network access."""

from __future__ import annotations

import re
import sys
from pathlib import Path
from urllib.parse import unquote

ROOT = Path(__file__).resolve().parent.parent
LINK_RE = re.compile(r"(?<!!)\[[^\]]*\]\(([^)]+)\)")
SKIP_PREFIXES = ("#", "http://", "https://", "mailto:")


def markdown_files() -> list[Path]:
    return sorted(
        path
        for path in ROOT.rglob("*.md")
        if ".git" not in path.parts and "target" not in path.parts
    )


def normalize_target(raw: str) -> str | None:
    target = raw.strip()
    if not target or target.startswith(SKIP_PREFIXES):
        return None
    if target.startswith("<") and target.endswith(">"):
        target = target[1:-1]
    target = unquote(target.split("#", 1)[0].strip())
    return target or None


def main() -> int:
    errors: list[str] = []
    for source in markdown_files():
        content = source.read_text(encoding="utf-8")
        for match in LINK_RE.finditer(content):
            target = normalize_target(match.group(1))
            if target is None or "://" in target:
                continue
            resolved = (source.parent / target).resolve()
            try:
                resolved.relative_to(ROOT)
            except ValueError:
                errors.append(
                    f"{source.relative_to(ROOT)}: link leaves repository: {match.group(1)}"
                )
                continue
            if not resolved.exists():
                errors.append(
                    f"{source.relative_to(ROOT)}: unresolved local link: {match.group(1)}"
                )

    if errors:
        print("\n".join(errors), file=sys.stderr)
        return 1

    print("markdown links: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
