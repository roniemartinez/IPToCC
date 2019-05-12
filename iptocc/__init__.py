#!/usr/bin/env python
# __author__ = "Ronie Martinez"
# __copyright__ = "Copyright 2017-2019, Ronie Martinez"
# __credits__ = ["Ronie Martinez"]
# __maintainer__ = "Ronie Martinez"
# __email__ = "ronmarti18@gmail.com"
import logging
import os
import threading
from ipaddress import ip_address, IPv4Address, IPv6Network

import pandas

from iptocc.exceptions import CountryCodeNotFound, CountryNotFound

try:
    from functools import lru_cache
except ImportError:
    from backports.functools_lru_cache import lru_cache

logger = logging.getLogger(__name__)

pandas.set_option('display.max_columns', None)
pandas.set_option('display.expand_frame_repr', False)
pandas.set_option('max_colwidth', -1)

lock = threading.Lock()
_rir_database = None  # type: pandas.DataFrame
_countries = dict()  # type: dict


def get_rir_database():
    global lock
    global _rir_database
    global _countries
    if _rir_database is None:
        with lock:
            if _rir_database is None:
                logger.info('Loading RIR databases')
                _rir_database = pandas.concat(read_rir_databases())
                _rir_database = _rir_database[((_rir_database['Type'] == 'ipv4') | (_rir_database['Type'] == 'ipv6')) &
                                              (_rir_database['Type'] != '*')]
                countries = pandas.read_csv(
                    os.path.join(os.path.dirname(os.path.abspath(__file__)), 'iso3166.csv'),
                    names=['country_code', 'country_name']
                )
                _countries = dict(zip(countries['country_code'].values, countries['country_name'].values))
                logger.info('RIR databases loaded')
    return _rir_database


def read_rir_databases():
    headers = ['Registry', 'Country Code', 'Type', 'Start', 'Value', 'Date', 'Status', 'Extensions']
    iptocc_dir = os.path.expanduser('~/.rir')
    for rir_database in os.listdir(iptocc_dir):
        if rir_database.startswith('delegated-') and rir_database.endswith('-extended-latest'):
            rir_database_path = os.path.join(iptocc_dir, rir_database)
            yield pandas.read_csv(rir_database_path, delimiter='|', comment='#', names=headers, dtype=str,
                                  keep_default_na=False, na_values=[''], encoding='utf-8')[4:]


@lru_cache(maxsize=100000)
def ipv4_get_country_code(address):
    rir_database = get_rir_database()  # pandas.DataFrame
    ipv4_database = rir_database[rir_database['Type'] == 'ipv4']
    for index, row in ipv4_database.iterrows():
        start_address = IPv4Address(row['Start'])
        if start_address <= address < start_address + int(row['Value']):
            return row['Country Code']
    raise CountryCodeNotFound


@lru_cache(maxsize=100000)
def ipv6_get_country_code(address):
    rir_database = get_rir_database()  # pandas.DataFrame
    ipv6_database = rir_database[rir_database['Type'] == 'ipv6']
    for index, row in ipv6_database.iterrows():
        if address in IPv6Network(row['Start'] + '/' + row['Value']):
            return row['Country Code']
    raise CountryCodeNotFound


def get_country_code(address):
    try:
        address = address.decode('utf-8')
    except AttributeError:
        pass
    address = ip_address(address)
    if isinstance(address, IPv4Address):
        logger.info("%s is IPv4", address)
        return ipv4_get_country_code(address)
    logger.info("%s is IPv6", address)
    return ipv6_get_country_code(address)


def get_country(address):
    global _countries
    try:
        country_code = get_country_code(address)
    except CountryCodeNotFound:
        raise CountryNotFound
    return _countries[country_code]
