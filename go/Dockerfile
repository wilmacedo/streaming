FROM golang:1.21-alpine3.18 AS build

WORKDIR /build

COPY . .

RUN go mod tidy

RUN GOOS=linux GOARCH=amd64 go build -o main

FROM alpine:latest

WORKDIR /dist

COPY --from=build /build/main /dist/main

COPY database.json /dist/database.json

CMD ["/dist/main"]