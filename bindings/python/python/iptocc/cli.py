import argparse
from importlib.metadata import version

from iptocc import country_code


def main() -> int:
    parser = argparse.ArgumentParser(
        prog="iptocc",
        description="Fast, offline IP-to-country lookup.",
    )
    parser.add_argument("addresses", nargs="*", help="IPv4 or IPv6 addresses to look up")
    parser.add_argument("--version", action="version", version=f"iptocc {version('iptocc')}")
    args = parser.parse_args()

    if not args.addresses:
        parser.print_help()
        return 0

    results = country_code(args.addresses)
    if len(args.addresses) == 1:
        if results[0] is None:
            return 1
        print(results[0])
        return 0

    for addr, result in zip(args.addresses, results, strict=True):
        print(f"{addr} {result or '-'}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
