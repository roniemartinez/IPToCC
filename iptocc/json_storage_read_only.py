#!/usr/bin/env python
from tinydb import JSONStorage
import logging

try:
    import ujson as json
except ImportError:
    import json

__author__ = "Ronie Martinez"
__copyright__ = "Copyright 2017, Ronie Martinez"
__credits__ = ["Ronie Martinez"]
__license__ = "MIT"
__version__ = "1.0.1"
__maintainer__ = "Ronie Martinez"
__email__ = "ronmarti18@gmail.com"
__status__ = "Prototype"

logger = logging.getLogger(__name__)


class JSONStorageReadOnly(JSONStorage):
    """
    Store the data in a JSON file.
    """

    def __init__(self, path):
        """
        Create a new instance.
        :param path: Where to store the JSON data.
        :type path: str
        """

        super(JSONStorage, self).__init__()
        self._handle = open(path, 'r')

    def close(self):
        self._handle.close()

    def read(self):
        return json.load(self._handle)

    def write(self, data):
        logger.debug('Read-only, ignoring write() call.')
