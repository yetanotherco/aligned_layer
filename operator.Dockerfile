FROM golang:1.22 as build

WORKDIR /usr/src/app

COPY go.mod go.sum ./

RUN go mod download && go mod tidy && go mod verify

COPY . .

#WORKDIR /usr/src/app/operator/cmd
RUN make build_all_ffi_linux
RUN go build -v -o /usr/local/bin/operator ./operator/cmd/main.go

FROM debian:latest
COPY --from=build /usr/local/bin/operator /usr/local/bin/operator
ENTRYPOINT [ "operator"]
CMD ["--config=/app/config-files/config-operator-1.yaml"]
