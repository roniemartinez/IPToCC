import argparse
import difflib
import json
import sys
from pathlib import Path

import semver
import tomlkit
from rich.console import Console
from rich.markup import escape

ROOT = Path(__file__).resolve().parent.parent
TARGETS = ("rust", "python", "wasm", "all")
KEYWORDS = (
    "patch",
    "minor",
    "major",
    "prepatch",
    "preminor",
    "premajor",
    "prerelease",
)

FILES: dict[str, list[Path]] = {
    "rust": [
        ROOT / "crate/Cargo.toml",
    ],
    "python": [
        ROOT / "bindings/python/Cargo.toml",
    ],
    "wasm": [
        ROOT / "bindings/wasm/Cargo.toml",
        ROOT / "bindings/wasm/package.json",
    ],
}

console = Console(highlight=False, soft_wrap=True)
err_console = Console(stderr=True, highlight=False, soft_wrap=True)


def _bump_prerelease(version: semver.Version, preid: str | None) -> semver.Version:
    if preid is None:
        return version.bump_prerelease()

    existing = version.prerelease.split(".")[0] if version.prerelease else None

    if existing is not None and existing != preid:
        version = version.replace(prerelease=None)

    return version.bump_prerelease(token=preid)


def bump(current: str, keyword: str, preid: str | None = None) -> str:
    version = semver.Version.parse(current)

    match keyword:
        case "patch":
            return str(version.bump_patch())
        case "minor":
            return str(version.bump_minor())
        case "major":
            return str(version.bump_major())
        case "prepatch":
            return str(_bump_prerelease(version.bump_patch(), preid))
        case "preminor":
            return str(_bump_prerelease(version.bump_minor(), preid))
        case "premajor":
            return str(_bump_prerelease(version.bump_major(), preid))
        case "prerelease":
            return str(_bump_prerelease(version, preid))
        case _:
            raise ValueError(f"unknown bump keyword: {keyword}")


def can_bulk_bump(keyword: str, versions: dict[str, str]) -> bool:
    parsed = [semver.Version.parse(v) for v in versions.values()]

    match keyword:
        case "patch" | "prepatch" | "prerelease":
            return len(set(versions.values())) == 1
        case "minor" | "preminor":
            return len({(p.major, p.minor) for p in parsed}) == 1
        case "major" | "premajor":
            return len({p.major for p in parsed}) == 1
        case _:
            raise ValueError(f"unknown bump keyword: {keyword}")


def read_version(path: Path) -> str:
    return str(tomlkit.parse(path.read_text())["package"]["version"])


def compute_updated_content(path: Path, new: str) -> str:
    if path.suffix == ".json":
        data = json.loads(path.read_text())
        data["version"] = new
        return json.dumps(data, indent=2) + "\n"

    doc = tomlkit.parse(path.read_text())
    doc["package"]["version"] = new
    return tomlkit.dumps(doc)


def resolve(args: argparse.Namespace) -> tuple[dict[str, str], str]:
    if args.target == "all":
        currents = {t: read_version(FILES[t][0]) for t in ("rust", "python", "wasm")}
    else:
        currents = {args.target: read_version(FILES[args.target][0])}

    if args.bump not in KEYWORDS:
        if args.target == "all" and len(set(currents.values())) > 1:
            raise RuntimeError(
                f"cannot set explicit version '{args.bump}' across diverged versions {currents}; "
                "bump each target individually",
            )
        return currents, str(semver.Version.parse(args.bump))

    if args.target == "all" and not can_bulk_bump(args.bump, currents):
        raise RuntimeError(
            f"cannot bulk bump '{args.bump}' across diverged versions {currents}; bump each target individually",
        )

    return currents, bump(next(iter(currents.values())), args.bump, args.preid)


def print_diff(path: Path, new: str) -> None:
    rel = str(path.relative_to(ROOT))
    old_text = path.read_text()
    new_text = compute_updated_content(path, new)
    diff = difflib.unified_diff(
        old_text.splitlines(keepends=True),
        new_text.splitlines(keepends=True),
        fromfile=rel,
        tofile=rel,
    )
    for line in diff:
        stripped = line.rstrip()
        safe = escape(stripped)

        if stripped.startswith("---") or stripped.startswith("+++"):
            console.print(f"[bold]{safe}[/bold]")
        elif stripped.startswith("@@"):
            console.print(f"[cyan]{safe}[/cyan]")
        elif stripped.startswith("+"):
            console.print(f"[green]{safe}[/green]")
        elif stripped.startswith("-"):
            console.print(f"[red]{safe}[/red]")
        else:
            console.print(safe)


def apply_to_file(path: Path, current: str, new: str, dry_run: bool) -> None:
    if dry_run:
        print_diff(path, new)
        return

    path.write_text(compute_updated_content(path, new))
    console.print(f"{path.relative_to(ROOT)}: {current} -> {new}")


def main() -> int:
    parser = argparse.ArgumentParser(description="Bump the version of one or all iptocc bindings.")
    parser.add_argument("target", choices=TARGETS, help=f"one of: {', '.join(TARGETS)}")
    parser.add_argument(
        "bump",
        metavar="BUMP",
        help=f"a version keyword ({', '.join(KEYWORDS)}) or an explicit version like 0.2.0-alpha.1",
    )
    parser.add_argument(
        "--preid",
        metavar="NAME",
        help="pre-release identifier (alpha, beta, rc) for prepatch/preminor/premajor/prerelease bumps",
    )
    parser.add_argument("--dry-run", action="store_true", help="print the changes without writing files")
    args = parser.parse_args()

    try:
        currents, new = resolve(args)
    except (ValueError, RuntimeError, KeyError) as e:
        err_console.print(f"[red]error:[/red] {e}")
        return 1

    for target, current in currents.items():
        for path in FILES[target]:
            apply_to_file(path, current, new, args.dry_run)

    return 0


if __name__ == "__main__":
    sys.exit(main())
