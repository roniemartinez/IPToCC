#!/usr/bin/env python
# __author__ = "Ronie Martinez"
# __copyright__ = "Copyright 2017-2018, Ronie Martinez"
# __credits__ = ["Ronie Martinez"]
# __maintainer__ = "Ronie Martinez"
# __email__ = "ronmarti18@gmail.com"
# __status__ = "Production"
import logging
import os
import sys
import threading

import pandas
from ipaddress import ip_address, IPv4Address, IPv6Network

try:
    from functools import lru_cache
except ImportError:
    # noinspection PyUnresolvedReferences
    from backports.functools_lru_cache import lru_cache

logging.basicConfig(stream=sys.stdout, level=logging.DEBUG,
                    format='%(asctime)s\t%(levelname)s\t%(module)s\t%(message)s')
logger = logging.getLogger('iptocc')

pandas.set_option('display.max_columns', None)
pandas.set_option('display.expand_frame_repr', False)
pandas.set_option('max_colwidth', -1)

lock = threading.Lock()
_rir_database = None  # type: pandas.DataFrame


class CountryCodeNotFound(Exception):
    pass


def convert_to_ip_object(row):
    if row['Type'] == 'ipv4':
        start = IPv4Address(row['Start'])
        return start, start + int(row['Value'])
    elif row['Type'] == 'ipv6':
        return IPv6Network(row['Start'] + '/' + row['Value']), ''
    else:
        return row['Start'], ''


def get_rir_database():
    global lock
    global _rir_database
    if _rir_database is None:
        with lock:
            if _rir_database is None:
                logger.info('Loading RIR databases')
                _rir_database = pandas.concat(read_rir_databases())
                _rir_database = _rir_database[((_rir_database['Type'] == 'ipv4') | (_rir_database['Type'] == 'ipv6')) &
                                              (_rir_database['Type'] != '*')]
                _rir_database[['Start', 'End']] = _rir_database.apply(convert_to_ip_object, axis=1,
                                                                      result_type='expand')
                logger.info('RIR databases loaded')
    return _rir_database


def read_rir_databases():
    headers = ['Registry', 'Country Code', 'Type', 'Start', 'Value', 'Date', 'Status', 'Extensions']
    iptocc_dir = os.path.dirname(os.path.abspath(__file__))
    for rir_database in os.listdir(iptocc_dir):
        if rir_database.startswith('delegated-') and rir_database.endswith('-extended-latest'):
            rir_database_path = os.path.join(iptocc_dir, rir_database)
            yield pandas.read_csv(rir_database_path, delimiter='|', comment='#', names=headers, dtype=str,
                                  keep_default_na=False, na_values=[''], encoding='utf-8')[4:]


@lru_cache(maxsize=100000)
def ipv4_get_country_code(address):
    rir_database = get_rir_database()  # pandas.DataFrame
    ipv4_database = rir_database[rir_database['Type'] == 'ipv4']
    result = ipv4_database[(ipv4_database['Start'] <= address) & (ipv4_database['End'] > address)]
    try:
        return result['Country Code'].tolist()[0]
    except IndexError:
        raise CountryCodeNotFound


@lru_cache(maxsize=100000)
def ipv6_get_country_code(address):
    rir_database = get_rir_database()  # pandas.DataFrame
    ipv6_database = rir_database[rir_database['Type'] == 'ipv6']
    result = ipv6_database[ipv6_database.apply(lambda row: address in row['Start'], axis=1, result_type='expand')]
    try:
        return result['Country Code'].tolist()[0]
    except IndexError:
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
