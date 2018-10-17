# IPToCC

 Get ISO country code of IPv4/IPv6 address. Address lookup is done locally.

<table>
    <tr>
        <td>License</td>
        <td><img src='https://img.shields.io/pypi/l/IPToCC.svg'></td>
        <td>Version</td>
        <td><img src='https://img.shields.io/pypi/v/IPToCC.svg'></td>
    </tr>
    <tr>
        <td>Travis CI</td>
        <td><img src='https://travis-ci.org/Code-ReaQtor/IPToCC.svg?branch=master'></td>
        <td>Coverage</td>
        <td><img src='https://codecov.io/gh/Code-ReaQtor/IPToCC/branch/master/graph/badge.svg'></td>
    </tr>
    <tr>
        <td>AppVeyor</td>
        <td><img src='https://ci.appveyor.com/api/projects/status/1xmd0gr278bhu090/branch/master?svg=true'></td>
        <td>Supported versions</td>
        <td><img src='https://img.shields.io/pypi/pyversions/IPToCC.svg'></td>
    </tr>
    <tr>
        <td>Wheel</td>
        <td><img src='https://img.shields.io/pypi/wheel/IPToCC.svg'></td>
        <td>Implementation</td>
        <td><img src='https://img.shields.io/pypi/implementation/IPToCC.svg'></td>
    </tr>
    <tr>
        <td>Status</td>
        <td><img src='https://img.shields.io/pypi/status/IPToCC.svg'></td>
        <td>Show your support</td>
        <td><a href='https://saythanks.io/to/Code-ReaQtor'><img src='https://img.shields.io/badge/Say%20Thanks-!-1EAEDB.svg'></a></td>
    </tr>
</table>

## Features

- [x] No external API call
- [x] No paid GeoIP service
- [x] Thread-safe
- [x] Offline

To learn about using IP addresses for geolocation, read the [Wikipedia article](https://en.wikipedia.org/wiki/Geolocation_software) to gain a basic understanding.

## Install

```bash
pip install IPToCC
```

## Usage

```python
from iptocc import get_country_code
country_code = get_country_code('<IPv4/IPv6 address>')
```

## Databases

- ftp://ftp.afrinic.net/stats/afrinic/delegated-afrinic-extended-latest
- ftp://ftp.arin.net/pub/stats/arin/delegated-arin-extended-latest
- ftp://ftp.apnic.net/public/apnic/stats/apnic/delegated-apnic-extended-latest
- ftp://ftp.lacnic.net/pub/stats/lacnic/delegated-lacnic-extended-latest
- ftp://ftp.ripe.net/pub/stats/ripencc/delegated-ripencc-extended-latest

## Dependencies

- [pandas](https://github.com/pandas-dev/pandas)
- [ipaddress](https://github.com/phihag/ipaddress)
- [backports.functools_lru_cache import lru_cache](https://github.com/jaraco/backports.functools_lru_cache)

## References

- [RIR Statistics Exchange Format](https://www.apnic.net/about-apnic/corporate-documents/documents/resource-guidelines/rir-statistics-exchange-format/)
- [How can I compile an IP address to country lookup database to make available for free?](https://webmasters.stackexchange.com/questions/34628/how-can-i-compile-an-ip-address-to-country-lookup-database-to-make-available-for)

## Maintainers

- [Ronie Martinez](mailto:ronmarti18@gmail.com)
