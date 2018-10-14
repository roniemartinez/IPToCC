#!/usr/bin/env python
# __author__ = "Ronie Martinez"
# __copyright__ = "Copyright 2017-2018, Ronie Martinez"
# __credits__ = ["Ronie Martinez"]
# __maintainer__ = "Ronie Martinez"
# __email__ = "ronmarti18@gmail.com"
# __status__ = "Production"
# noinspection PyPackageRequirements
import pytest

from iptocc import CountryCodeNotFound, get_country_code


def test_ipv4_private():
    with pytest.raises(CountryCodeNotFound):
        get_country_code('10.0.0.0')


def test_ipv4_us():
    assert 'US' == get_country_code('5.35.192.0')


def test_ipv4_se():
    assert 'SE' == get_country_code('5.35.184.0')


def test_ipv6_us():
    assert 'US' == get_country_code('2a00:5440:0000:0000:0000:ff00:0042:8329')


def test_ipv6_gb():
    assert 'GB' == get_country_code('2a00:95e0:0000:0000:0000:ff00:0042:8329')


def test_invalid_ip():
    with pytest.raises(ValueError):
        get_country_code('123456')
