#!/usr/bin/env python3
"""Inline Rust module tree and extract struct definitions from corepc-types."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path
from typing import Optional

_MOD_RE = re.compile(
    r'^(?P<attrs>(?:\s*#\[[^\]]+\]\s*)*)\s*'
    r'(?P<vis>pub(?:\([^)]*\))?\s+)?'
    r'mod\s+(?P<name>[a-zA-Z_]\w*)\s*;\s*$'
)

_SKIP_BLOCK_RE = re.compile(r'^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(into|error)\s*\{')


def _find_mod(base: Path, name: str) -> Path:
    for candidate in (base / f"{name}.rs", base / name / "mod.rs"):
        if candidate.exists():
            return candidate
    raise FileNotFoundError(f"Cannot find module `{name}` under {base}")


def _inline_mod(path: Path, seen: set[Path]) -> str:
    path = path.resolve()
    if path in seen:
        return ""
    seen.add(path)

    out = []
    for line in path.read_text().splitlines(True):
        m = _MOD_RE.match(line)
        if not m:
            out.append(line)
            continue
        attrs = m.group("attrs") or ""
        vis = m.group("vis") or ""
        mod_path = _find_mod(path.parent, m.group("name"))
        out.append(attrs)
        out.append(f"{vis}mod {m.group('name')} {{\n")
        body = _inline_mod(mod_path, seen)
        out.append("".join("    " + l if l.strip() else l for l in body.splitlines(True)))
        out.append("}\n")
    return "".join(out)


def _extract_structs(text: str) -> str:
    """Keep only pub struct/enum definitions, stripping impl blocks, use, mod wrappers."""
    lines = text.splitlines(keepends=True)
    result = []
    depth = 0      # brace depth for blocks we're skipping
    in_struct = False
    struct_depth = 0
    i = 0

    while i < len(lines):
        s = lines[i].strip()

        # Inside a block we're skipping
        if depth > 0:
            depth += s.count('{') - s.count('}')
            i += 1
            continue

        # Skip: mod into/error {}, impl blocks, #[cfg] impl
        if _SKIP_BLOCK_RE.match(s) or re.match(r'^\s*impl\s+', s):
            depth = s.count('{') - s.count('}')
            i += 1
            continue
        if s == '#[cfg(feature = "std")]' and i + 1 < len(lines) and 'impl' in lines[i + 1]:
            i += 1
            depth = lines[i].strip().count('{') - lines[i].strip().count('}')
            i += 1
            continue

        # Skip: use statements, mod wrappers, SPDX, module docs
        if re.match(r'^\s*(pub\s+)?use\s+', s):
            while i < len(lines) and not lines[i].rstrip().endswith(';'):
                i += 1
            i += 1
            continue
        if re.match(r'^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*\{\s*$', s):
            i += 1
            continue
        if s.startswith('// SPDX') or s.startswith('//!'):
            i += 1
            continue

        # Track struct/enum blocks
        if re.match(r'^(pub\s+)?(struct|enum)\s+\w+', s):
            in_struct = True
            struct_depth = 0

        if in_struct:
            struct_depth += s.count('{') - s.count('}')

        if s == '}':
            if in_struct and struct_depth <= 0:
                in_struct = False
                result.append(s + "\n")
                i += 1
                continue
            else:
                i += 1
                continue

        if not s:
            i += 1
            continue

        result.append(lines[i].lstrip())
        i += 1

    # Collapse consecutive blank lines
    out, prev_blank = [], False
    for line in result:
        blank = not line.strip()
        if blank and prev_blank:
            continue
        out.append(line)
        prev_blank = blank
    return "".join(out)


def _collect_all_versions(types_src: Path, up_to: int | None = None) -> str:
    """Walk types/src/v17..vN, extract structs, keep latest version of each."""
    structs: dict[str, str] = {}  # name -> definition

    for vdir in sorted(types_src.iterdir()):
        if not vdir.is_dir() or not vdir.name.startswith('v'):
            continue
        try:
            ver = int(vdir.name[1:])
        except ValueError:
            continue
        if up_to is not None and ver > up_to:
            continue

        mod_file = vdir / "mod.rs"
        if not mod_file.exists():
            continue

        text = _extract_structs(_inline_mod(mod_file, set()))

        name, body = None, []
        for line in text.splitlines(keepends=True):
            m = re.match(r'^pub struct (\w+)', line)
            if m:
                if name and body:
                    structs[name] = "".join(body)
                name = m.group(1)
                body = [line]
            elif name:
                body.append(line)
                s = line.strip()
                if s == '}' or (s.endswith(');') and '(' in "".join(body)):
                    structs[name] = "".join(body)
                    name, body = None, []
            elif line.strip().startswith('///') or line.strip().startswith('#['):
                body.append(line)
            else:
                body = []

        if name and body:
            structs[name] = "".join(body)

    return "".join(structs[n] + "\n" for n in sorted(structs))


def main() -> int:
    ap = argparse.ArgumentParser(description="Inline Rust modules and extract structs")
    ap.add_argument("entry", nargs='?', help="Entry .rs file")
    ap.add_argument("-o", "--out", default="inlined.rs")
    ap.add_argument("--extract-structs", action="store_true")
    ap.add_argument("--all-versions", action="store_true")
    ap.add_argument("--types-dir", default=None)
    ap.add_argument("--up-to-version", type=int, default=None)
    args = ap.parse_args()

    if args.all_versions:
        types_dir = Path(args.types_dir) if args.types_dir else Path("types/src")
        if not types_dir.exists():
            print(f"Error: {types_dir} not found", file=sys.stderr)
            return 1
        text = _collect_all_versions(types_dir, args.up_to_version)
    else:
        if not args.entry:
            print("Error: entry file required unless --all-versions", file=sys.stderr)
            return 1
        text = _inline_mod(Path(args.entry), set())
        if args.extract_structs:
            text = _extract_structs(text)

    Path(args.out).write_text(text)
    return 0


if __name__ == "__main__":
    sys.exit(main())
