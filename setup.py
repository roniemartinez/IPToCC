#!/usr/bin/env python
import ssl

import sys
from setuptools import setup, find_packages

ssl._create_default_https_context = ssl._create_unverified_context

VERSION = open('iptocc/VERSION').read().strip()
REQUIREMENTS = []
with open('iptocc/requirements.txt') as f:
    for line in f:
        REQUIREMENTS.append(line.strip())
if sys.version_info[0] == 2:
    REQUIREMENTS += ['backports.functools_lru_cache', 'ipaddress']

setup(
    name='IPToCC',
    version=VERSION,
    packages=find_packages(),
    url='https://github.com/Code-ReaQtor/IPToCC',
    download_url='https://github.com/Code-ReaQtor/IPToCC/tarball/{}'.format(VERSION),
    license='MIT',
    author='Ronie Martinez',
    author_email='ronmarti18@gmail.com',
    description='Get country code of IPv4/IPv6 address. Address lookup is done offline.',
    long_description=open('README').read(),
    classifiers=[
        'Development Status :: 4 - Beta',
        'License :: OSI Approved :: MIT License',
        'Topic :: Software Development :: Libraries :: Python Modules',
        'Programming Language :: Python',
        'Programming Language :: Python :: 2',
        'Programming Language :: Python :: 3',
    ],
    setup_requires=['pytest-runner'],
    install_requires=REQUIREMENTS,
    tests_require=['pytest', 'pytest-cov', 'codecov'],
    package_data={
        'iptocc': ['rir_statistics_exchange.json', 'VERSION', 'requirements.txt'],
        '': ['setup.cfg', 'README']
    }
)
