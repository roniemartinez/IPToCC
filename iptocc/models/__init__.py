#!/usr/bin/env python
from sqlalchemy import Column, Integer, String
from sqlalchemy.ext.declarative import declarative_base

__author__ = "Ronie Martinez"
__copyright__ = "Copyright 2017, Ronie Martinez"
__credits__ = ["Ronie Martinez"]
__license__ = "MIT"
__version__ = "1.0.1"
__maintainer__ = "Ronie Martinez"
__email__ = "ronmarti18@gmail.com"
__status__ = "Prototype"


Base = declarative_base()


class Record(Base):
    __tablename__ = 'record'

    id = Column(Integer, nullable=False, primary_key=True)
    country_code = Column(String(2), nullable=False)
    type = Column(String, nullable=False)
    start = Column(String, nullable=False)
    value = Column(Integer, nullable=False)

    def __repr__(self):
        return '<Record {} {} {} {} {}>'.format(self.registry, self.country_code, self.type, self.start, self.value)
