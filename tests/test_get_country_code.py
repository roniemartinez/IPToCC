#!/usr/bin/env python
# __author__ = "Ronie Martinez"
# __copyright__ = "Copyright 2017-2020, Ronie Martinez"
# __credits__ = ["Ronie Martinez"]
# __maintainer__ = "Ronie Martinez"
# __email__ = "ronmarti18@gmail.com"
import pytest

from iptocc import CountryCodeNotFound, get_country_code

PARAMS = [
    ("United States - IPv4", "5.35.192.0", "US"),
    ("Sweden - IPv4", "5.35.184.0", "SE"),
    ("United States - IPv6", "2a00:5440:0000:0000:0000:ff00:0042:8329", "US",),
    ("United Kingdom - IPv6", "2a00:95e0:0000:0000:0000:ff00:0042:8329", "GB",),
]


@pytest.mark.parametrize(
    "name, ip, expected", ids=[x[0] for x in PARAMS], argvalues=PARAMS
)
def test_get_country_code(name, ip, expected):
    assert get_country_code(ip) == expected, name


def test_ipv4_private():
    with pytest.raises(CountryCodeNotFound):
        get_country_code("10.0.0.0")


def test_invalid_ip():
    with pytest.raises(ValueError):
        get_country_code("123456")
