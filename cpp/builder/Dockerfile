FROM golang:1.15-alpine AS builder

WORKDIR /go/src/app
RUN apk --no-cache add git

RUN go get cloud.google.com/go/storage
COPY main.go ./main.go
RUN go build main.go

FROM alpine:3.13.5

WORKDIR /app
RUN apk --no-cache add git curl gcompat

RUN curl -L -O https://github.com/protocolbuffers/protobuf/releases/download/v3.15.8/protoc-3.15.8-linux-x86_64.zip && \
    unzip protoc-3.15.8-linux-x86_64.zip && \
    rm -rf protoc-3.15.8-linux-x86_64.zip include readme.txt && \
    mv bin/protoc /usr/bin/protoc && \
    rm -rf bin

RUN protoc --version

COPY --from=builder /go/src/app/main /app/main

CMD ["/app/main"]