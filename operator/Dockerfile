FROM golang:1.22.2 as builder

WORKDIR /app

COPY . .

RUN CGO_ENABLED=0 GOOS=linux go build -o operator ./cmd/main.go

FROM scratch

COPY --from=builder /app/operator /operator

ENTRYPOINT ["/operator"]
