# build stage
FROM golang:1.21-alpine3.18 AS build

# set working directory
WORKDIR /app

# copy source code
COPY . .

# install dependencies
RUN go mod download

# build binary
RUN go build -o /app

EXPOSE 8000

# run
CMD ["/app"]