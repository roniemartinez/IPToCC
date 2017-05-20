#!/usr/bin/env python
import ssl
from distutils.core import setup

ssl._create_default_https_context = ssl._create_unverified_context

VERSION = open('VERSION').read().strip()
REQUIREMENTS = []
with open('requirements.txt') as f:
    for line in f:
        REQUIREMENTS.append(line.strip())

setup(
    name='IPToCC',
    version=VERSION,
    packages=['iptocc', 'iptocc.models'],
    url='https://github.com/Code-ReaQtor/IPToCC',
    download_url='https://github.com/Code-ReaQtor/IPToCC/tarball/{}'.format(VERSION),
    license='MIT',
    author='Ronie Martinez',
    author_email='ronmarti18@gmail.com',
    description='Get country code of IPv4/IPv6 address. Address lookup is done offline.',
    long_description=open('README').read(),
    classifiers=[
        'Development Status :: 3 - Alpha',
        'License :: OSI Approved :: MIT License',
        'Topic :: Software Development :: Libraries :: Python Modules',
        'Programming Language :: Python',
        'Programming Language :: Python :: 3',
    ],
    install_requires=REQUIREMENTS,
    package_data={'iptocc': ['rir_statistics_exchange.db']}
)
