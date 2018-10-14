#!/usr/bin/env python
from setuptools import setup

VERSION = '2.0.0'

setup(
    name='IPToCC',
    version=VERSION,
    packages=['iptocc'],
    url='https://github.com/Code-ReaQtor/IPToCC',
    download_url='https://github.com/Code-ReaQtor/IPToCC/tarball/{}'.format(VERSION),
    license='MIT',
    author='Ronie Martinez',
    author_email='ronmarti18@gmail.com',
    description='Get country code of IPv4/IPv6 address. Address lookup is done offline.',
    long_description=open('README.md').read(),
    long_description_content_type='text/markdown',
    keywords=[],
    install_requires=['pandas==0.23.4'],
    extras_require={
        ':python_version < "3.2"': [
            'backports.functools-lru-cache==1.5'
        ],
        ':python_version < "3.3"': [
            'ipaddress==1.0.22'
        ]
    },
    classifiers=['Development Status :: 5 - Production/Stable',
                 'License :: OSI Approved :: MIT License',
                 'Topic :: Software Development :: Libraries :: Python Modules',
                 'Programming Language :: Python :: 2.7',
                 'Programming Language :: Python :: 3.5',
                 'Programming Language :: Python :: 3.6',
                 'Programming Language :: Python :: 3.7',
                 'Topic :: Scientific/Engineering :: Mathematics'],
    package_data={'iptocc': ['delegated-afrinic-extended-latest',
                             'delegated-arin-extended-latest',
                             'delegated-apnic-extended-latest',
                             'delegated-lacnic-extended-latest',
                             'delegated-ripencc-extended-latest'
                             ]
                  }
)
