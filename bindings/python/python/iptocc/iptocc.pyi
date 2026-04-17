from collections.abc import Iterable
from typing import overload

@overload
def country_code(addr: str) -> str | None: ...  # type: ignore[overload-overlap]
@overload
def country_code(addr: Iterable[str]) -> list[str | None]: ...
