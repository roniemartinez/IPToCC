# IPToCC

[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg)](https://github.com/RichardLitt/standard-readme)
[![PyPI](https://img.shields.io/pypi/v/nine.svg)](https://pypi.org/project/IPToCC/)
[![PyPI - License](https://img.shields.io/pypi/l/IPToCC.svg)](https://pypi.org/project/IPToCC/)
[![PyPI - Status](https://img.shields.io/pypi/status/IPToCC.svg)](https://pypi.org/project/IPToCC/)
[![PyPI - Implementation](https://img.shields.io/pypi/implementation/IPToCC.svg)](https://pypi.org/project/IPToCC/)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/IPToCC.svg)](https://pypi.org/project/IPToCC/)
[![Build Status](https://travis-ci.org/tatemz/IPToCC.svg?branch=master)](https://travis-ci.org/tatemz/IPToCC)
[![codecov](https://codecov.io/gh/tatemz/IPToCC/branch/master/graph/badge.svg)](https://codecov.io/gh/tatemz/IPToCC)

> Get ISO country code of IPv4/IPv6 address. Address lookup is done locally.

- No external API call.
- No paid GeoIP service.

## Table of Contents
- [Table of Contents](#table-of-contents)
- [Background](#background)
    - [Features](#features)
- [Install](#install)
- [Usage](#usage)
- [Sources](#sources)
- [Libraries Used](#libraries-used)
- [Old implementations](#old-implementations)
- [References](#references)
- [Maintainers](#maintainers)
- [Contribution](#contribution)
    - [Install Dependencies](#install-dependencies)
    - [Testing](#testing)
    - [Build Geolocation Database](#build-geolocation-database)
    - [Docker Environment](#docker-environment)
- [License](#license)

## Background

To learn about using IP addresses for geolocation, read the [Wikipedia article](https://en.wikipedia.org/wiki/Geolocation_software) to gain a basic understanding.

Also read [The Free and Simple Way To Know Who Visits Your Site](roniemartinez.space/blog/the_free_and_simple_way_to_know_who_visits_your_site).

### Features

- Thread-safe
- Offline

## Install

```bash
pip install IPToCC
```

## Usage

```python
import iptocc
country_code = iptocc.get_country_code('<IPv4/IPv6 address>')
```

## Sources

The static database that comes with this library is built from the following sources

- ftp://ftp.afrinic.net/stats/afrinic/delegated-afrinic-extended-latest
- ftp://ftp.arin.net/pub/stats/arin/delegated-arin-extended-latest
- ftp://ftp.apnic.net/public/apnic/stats/apnic/delegated-apnic-extended-latest
- ftp://ftp.lacnic.net/pub/stats/lacnic/delegated-lacnic-extended-latest
- ftp://ftp.ripe.net/pub/stats/ripencc/delegated-ripencc-extended-latest

## Libraries Used

- [TinyDB](https://github.com/msiemens/tinydb)
- [UltraJSON](https://github.com/esnme/ultrajson)

## Old implementations

- [SQLAlchemy](https://www.sqlalchemy.org/) + SQLite - Not thread-safe
- [UnQLite](https://github.com/coleifer/unqlite-python) - large database file, problems with thread-safety

## References

- [RIR Statistics Exhange Format](https://www.apnic.net/about-apnic/corporate-documents/documents/resource-guidelines/rir-statistics-exchange-format/)
- [How can I compile an IP address to country lookup database to make available for free?](https://webmasters.stackexchange.com/questions/34628/how-can-i-compile-an-ip-address-to-country-lookup-database-to-make-available-for)

## Maintainers

- [Ronie Martinez](mailto:ronmarti18@gmail.com)

## Contribution

### Install Dependencies

To setup your project, run the following `setup.py` script:

```sh
python setup.py install
```

### Testing

To test your project, run the following `setup.py` script:

```sh
python setup.py test
```

### Build Geolocation Database

To build the database, run the following `database_builder.py` script:

```sh
python database_builder.py
```

### Docker Environment

This repo contains a Dockerfile in the case that a local development environment is needed to install the dependencies, build the database, as well as run tests in both Python 2 and Python 3.

To build the Docker image, see the following two example commands:

```sh
docker build --build-arg VERSION=2 -t iptocc:v2 ./
docker build --build-arg VERSION=3 -t iptocc:v3 ./
```

To build the Docker images without running the `database_builder.py` script, pass `--build-arg SKIP_BUILD_DB=true` to the build command.


The default command that is run within the Docker image is `python setup.py test`. To run the tests within a Docker container, see the following two example commands:

```sh
docker run --rm -it -e PYTHONPATH=. iptocc:v2
docker run --rm -it -e PYTHONPATH=. iptocc:v3
```

## License

[MIT](LICENSE) © Ronie Martinez
