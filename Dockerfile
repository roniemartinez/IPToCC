ARG VERSION=3
FROM python:$VERSION
ARG VERSION
WORKDIR /usr/src/app
ADD ./iptocc/requirements-v${VERSION}.txt /usr/src/app/iptocc/requirements-v${VERSION}.txt
RUN pip install -r /usr/src/app/iptocc/requirements-v${VERSION}.txt
ADD ./iptocc /usr/src/app/iptocc
ADD ./test /usr/src/app/test
VOLUME [ "/usr/src/app" ]
