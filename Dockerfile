ARG VERSION=3
FROM python:$VERSION
ARG VERSION
WORKDIR /usr/src/app
COPY ./ /usr/src/app/
RUN python setup.py install
ARG SKIP_BUILD_DB=false
RUN if [ ! $SKIP_BUILD_DB = "true" ]; then python database_builder.py; fi
VOLUME [ "/usr/src/app" ]
CMD ["python", "setup.py", "test"]
