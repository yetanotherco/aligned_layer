package metrics

import (
	"context"
	"errors"
	"github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/types"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
	"github.com/prometheus/client_golang/prometheus/promhttp"
	"net/http"
)

type Metrics struct {
	ipPortAddress          string
	logger                 logging.Logger
	numAggregatedResponses prometheus.Counter
}

const alignedNamespace = "aligned"

func NewMetrics(ipPortAddress string, reg prometheus.Registerer, logger logging.Logger) *Metrics {
	return &Metrics{
		ipPortAddress: ipPortAddress,
		logger:        logger,
		numAggregatedResponses: promauto.With(reg).NewCounter(prometheus.CounterOpts{
			Namespace: alignedNamespace,
			Name:      "aggregated_responses",
			Help:      "Number of aggregated responses sent to the Aligned Service Manager",
		}),
	}
}

// Start creates a http handler for reg and starts the prometheus server in a goroutine, listening at m.ipPortAddress.
// reg needs to be the prometheus registry that was passed in the NewMetrics constructor
func (m *Metrics) Start(ctx context.Context, reg prometheus.Gatherer) <-chan error {
	m.logger.Infof("Starting metrics server at port %v", m.ipPortAddress)
	errC := make(chan error, 1)
	go func() {
		http.Handle("/metrics", promhttp.HandlerFor(
			reg,
			promhttp.HandlerOpts{},
		))
		err := http.ListenAndServe(m.ipPortAddress, nil)
		if err != nil {
			errC <- types.WrapError(errors.New("prometheus server failed"), err)
		} else {
			errC <- nil
		}
	}()
	return errC
}

func (m *Metrics) IncAggregatedResponses() {
	m.numAggregatedResponses.Inc()
}
