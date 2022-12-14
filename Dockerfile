FROM alpine as stripper

RUN apk add binutils
RUN apk --no-cache add ca-certificates

COPY casa_server /casa_server
RUN strip /casa_server

FROM scratch as run

COPY --from=stripper /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=stripper /casa_server /casa_server

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD ["/casa_server"]
