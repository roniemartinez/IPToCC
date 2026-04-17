import iptocc
import pytest


@pytest.mark.parametrize(
    ("address", "expected"),
    [
        ("41.0.0.1", "ZA"),
        ("2001:4200::1", "ZA"),
        ("1.0.16.1", "JP"),
        ("2001:200::1", "JP"),
        ("8.8.8.8", "US"),
        ("2001:4860:4860::8888", "US"),
        ("200.160.0.1", "BR"),
        ("2001:1280::1", "BR"),
        ("193.0.6.139", "NL"),
        ("2001:67c:18::1", "NL"),
        ("10.0.0.0", None),
        ("not-an-ip", None),
    ],
)
def test_country_code_single(address: str, expected: str | None):
    assert iptocc.country_code(address) == expected


@pytest.mark.parametrize(
    ("inputs", "expected"),
    [
        (["8.8.8.8", "1.0.16.1", "10.0.0.0"], ["US", "JP", None]),
        ([], []),
    ],
    ids=["mixed_hits_and_miss", "empty"],
)
def test_country_code_batch(inputs: list[str], expected: list[str | None]):
    assert iptocc.country_code(inputs) == expected
