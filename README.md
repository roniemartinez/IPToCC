# IPToCC

Get ISO country code of IPv4/IPv6 address. Address lookup is done locally.

- No external API call.
- No paid GeoIP service.

Read [The Free and Simple Way To Know Who Visits Your Site](roniemartinez.space/blog/the_free_and_simple_way_to_know_who_visits_your_site).

## Features

- Thread-safe

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

## Contribution

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

## Author

- [Ronie Martinez](mailto:ronmarti18@gmail.com)
