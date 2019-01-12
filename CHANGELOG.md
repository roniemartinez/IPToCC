# List of changes to IPToCC

## Unreleased

## 2.1.1 - 2019-01-13
### Changed
- Update copyright

## 2.1.0 - 2018-10-17
### Added
- Return country name (#5)

## 2.0.1 - 2018-10-17
### Added
- More Shields (#3)

## 2.0.0 - 2018-10-14
### Changed
- Pandas core (#3)

## 1.1.2 - 2018-05-24
### Fixed
- IPv6 cannot search database (#1)

## 1.1.1 - 2018-05-24
### Changed
- Using [JSONStorageReadOnly](https://github.com/msiemens/tinydb/issues/136).

### Added
- Python 2 support.
- Logger in debug mode.

## 1.0.8 - 2017-05-22
### Fixed
- Missing packages in requirements.txt.

## 1.0.7 - 2017-05-22
### Changed
- Changed from distutils.core to setuptools.

## 1.0.6 - 2017-05-22
### Changed
- Used TinyDB and threading.Lock().

## 1.0.5 - 2017-05-21
### Fixed
- Collection returns NoneType.

## 1.0.4 - 2017-05-21
### Removed
- Excluded iptocc/models from MANIFEST and setup.py.

## 1.0.3 - 2017-05-21
### Changed
- Used unqlite-python instead of SQLAlchemy(sqlite) for thread-safety.
- Used ftp://ftp.afrinic.net/stats/afrinic/delegated-afrinic-extended-latest.

## 1.0.2 - 2017-05-20
### Fixed
- Cannot add subdirs in MANIFEST.

## 1.0.1 - 2017-05-20
### Added
- Support for IPv4 and IPv6 addresses.
- PyPi setup.
