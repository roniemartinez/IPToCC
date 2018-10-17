#!/usr/bin/env python
# __author__ = "Ronie Martinez"
# __copyright__ = "Copyright 2017-2018, Ronie Martinez"
# __credits__ = ["Ronie Martinez"]
# __maintainer__ = "Ronie Martinez"
# __email__ = "ronmarti18@gmail.com"
# __status__ = "Development"
# noinspection PyPackageRequirements
import pytest

from iptocc import CountryNotFound, get_country


def test_ipv4_private():
    with pytest.raises(CountryNotFound):
        get_country('10.0.0.0')


def test_ipv4_united_states():
    assert 'United States' == get_country('5.35.192.0')


def test_ipv4_sweden():
    assert 'Sweden' == get_country('5.35.184.0')


def test_ipv6_united_states():
    assert 'United States' == get_country('2a00:5440:0000:0000:0000:ff00:0042:8329')


def test_ipv6_united_kingdom():
    assert 'United Kingdom' == get_country('2a00:95e0:0000:0000:0000:ff00:0042:8329')


def test_invalid_ip():
    with pytest.raises(ValueError):
        get_country('123456')
