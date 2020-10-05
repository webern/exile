# syntax=docker/dockerfile:1.1.3-experimental
FROM alpine:3.7 as smallinux
RUN apk add --no-cache curl unzip tar bash

FROM smallinux as tempdata
COPY ./pull-data.sh /pull-data.sh
RUN /pull-data.sh --outdir /data

FROM scratch
COPY --from=tempdata /data /data