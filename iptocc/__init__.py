#!/usr/bin/env python
import ipaddress
import os
from functools import lru_cache

from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker

from iptocc.models import Record, Base

__author__ = "Ronie Martinez"
__copyright__ = "Copyright 2017, Ronie Martinez"
__credits__ = ["Ronie Martinez"]
__license__ = "MIT"
__version__ = "1.0.2"
__maintainer__ = "Ronie Martinez"
__email__ = "ronmarti18@gmail.com"
__status__ = "Production"

dir_path = os.path.dirname(os.path.realpath(__file__))
engine = create_engine("sqlite:///{}".format(os.path.join(dir_path, 'rir_statistics_exchange.db')))
Session = sessionmaker(bind=engine)
session = Session()


@lru_cache(maxsize=100000)
def ipv4_get_country_code(ip_address):
    for record in session.query(Record).filter_by(type='ipv4'):
        start_address = ipaddress.IPv4Address(record.start)
        if start_address <= ip_address < start_address + record.value:
            return record.country_code
    return None


@lru_cache(maxsize=100000)
def ipv6_get_country_code(ip_address):
    for record in session.query(Record).filter_by(type='ipv6'):
        network = ipaddress.IPv6Network('{}/{}'.format(record.start, record.value))
        if ip_address in network:
            return record.country_code
    return None


def get_country_code(ip_address):
    if type(ip_address) is str:
        ip_address = ipaddress.ip_address(ip_address)  # convert to ipaddress.IPv4Address or ipaddress.IPv6Address
    if type(ip_address) is ipaddress.IPv4Address:
        return ipv4_get_country_code(ip_address)  # IPv4
    return ipv6_get_country_code(ip_address)  # IPv6
