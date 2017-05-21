#!/usr/bin/env python
import csv
import hashlib
import os
import urllib.request

import iptocc

__author__ = "Ronie Martinez"
__copyright__ = "Copyright 2017, Ronie Martinez"
__credits__ = ["Ronie Martinez"]
__license__ = "MIT"
__version__ = "1.0.1"
__maintainer__ = "Ronie Martinez"
__email__ = "ronmarti18@gmail.com"
__status__ = "Prototype"


def download_progress(count, block_size, total_size):
    print('\t{}%'.format(int(count * block_size * 100 / total_size)))


if __name__ == '__main__':

    for url in (
        'ftp://ftp.afrinic.net/stats/afrinic/delegated-afrinic-extended-latest',
        'ftp://ftp.arin.net/pub/stats/arin/delegated-arin-extended-latest',
        'ftp://ftp.apnic.net/public/apnic/stats/apnic/delegated-apnic-extended-latest',
        'ftp://ftp.lacnic.net/pub/stats/lacnic/delegated-lacnic-extended-latest',
        'ftp://ftp.ripe.net/pub/stats/ripencc/delegated-ripencc-extended-latest',
    ):
        filename = url.split('/')[-1]
        if os.path.isfile(filename):
            hash_md5 = hashlib.md5()
            with open(filename, 'rb') as f:
                for chunk in iter(lambda: f.read(4096), b''):
                    hash_md5.update(chunk)
            md5_text = urllib.request.urlopen(url + '.md5').read().decode('utf-8')
            calculated_md5 = hash_md5.hexdigest()
            if not(calculated_md5 != md5_text[-33:-1] or calculated_md5 != md5_text[:32]):
                print("downloading latest file {}".format(url))
                urllib.request.urlretrieve(url, filename, download_progress)
        else:
            print("downloading file {}".format(filename))
            urllib.request.urlretrieve(url, filename, download_progress)
        print("reading file {}".format(filename))
        with open(filename, 'r') as f:
            record_reader = csv.reader(f, delimiter='|', quoting=csv.QUOTE_NONE)
            records = []
            for row in record_reader:
                if row[0] in ('ripencc', 'lacnic', 'arin', 'apnic', 'afrinic') \
                        and row[-1] != 'summary' \
                        and row[6] in ('allocated', 'assigned')\
                        and row[2] in ('ipv4', 'ipv6'):
                    country_code = row[1]
                    type_ = row[2]
                    start = row[3]
                    value = int(row[4])
                    collection = iptocc.database.collection(type_)
                    if not collection.exists():
                        collection.create()
                    collection.store({
                        'country_code': country_code,
                        'type': type_,
                        'start': start,
                        'value': value
                    })
            iptocc.database.commit()
