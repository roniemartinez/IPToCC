#!/usr/bin/env python
import csv
import hashlib
import os
import threading
from six.moves import urllib

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


def process(url_):
    filename = url_.split('/')[-1]
    if os.path.isfile(filename):
        hash_md5 = hashlib.md5()
        with open(filename, 'rb') as f:
            for chunk in iter(lambda: f.read(4096), b''):
                hash_md5.update(chunk)
        md5_text = urllib.request.urlopen(url_ + '.md5').read().decode('utf-8')
        calculated_md5 = hash_md5.hexdigest()
        if not (calculated_md5 != md5_text[-33:-1] or calculated_md5 != md5_text[:32]):
            print("downloading latest file {}".format(url_))
            urllib.request.urlretrieve(url_, filename)
    else:
        print("downloading file {}".format(filename))
        urllib.request.urlretrieve(url_, filename)
    print("reading file {}".format(filename))

    with open(filename, 'r') as f:
        records = []
        record_reader = csv.reader(f, delimiter='|', quoting=csv.QUOTE_NONE)
        for row in record_reader:
            if row[0] in ('ripencc', 'lacnic', 'arin', 'apnic', 'afrinic') \
                    and row[-1] != 'summary' \
                    and row[6] in ('allocated', 'assigned') \
                    and row[2] in ('ipv4', 'ipv6'):
                country_code = row[1]
                type_ = row[2]
                start = row[3]
                value = int(row[4])
                records.append({
                    'country_code': country_code,
                    'type': type_,
                    'start': start,
                    'value': value
                })
        print("Saving {} records.".format(len(records)))
        iptocc.database.insert_multiple(records)


if __name__ == '__main__':
    threads = []
    for url in (
        'ftp://ftp.afrinic.net/stats/afrinic/delegated-afrinic-extended-latest',
        'ftp://ftp.arin.net/pub/stats/arin/delegated-arin-extended-latest',
        'ftp://ftp.apnic.net/public/apnic/stats/apnic/delegated-apnic-extended-latest',
        'ftp://ftp.lacnic.net/pub/stats/lacnic/delegated-lacnic-extended-latest',
        'ftp://ftp.ripe.net/pub/stats/ripencc/delegated-ripencc-extended-latest',
    ):
        thread = threading.Thread(target=process, args=(url,))
        threads.append(thread)
        thread.start()
    for thread in threads:
        thread.join()
    print("Flushing cache.")
    iptocc.caching_middleware.flush()
