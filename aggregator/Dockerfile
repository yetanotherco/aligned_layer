FROM golang:1.22.2

WORKDIR /app

COPY go.mod go.sum ./

RUN go mod download

COPY . .


COPY ../Makefile .

COPY contracts/ ../contracts/
COPY operator/ ../operator/

CMD ["make", "aggregator-start"]
