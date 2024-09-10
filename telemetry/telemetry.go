package telemetry

import (
	"context"
	"encoding/hex"
	"fmt"
	"github.com/Layr-Labs/eigensdk-go/logging"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/exporters/otlp/otlpmetric/otlpmetricgrpc"
	"go.opentelemetry.io/otel/exporters/otlp/otlptrace/otlptracegrpc"
	"go.opentelemetry.io/otel/metric"
	"go.opentelemetry.io/otel/propagation"
	sdkmetric "go.opentelemetry.io/otel/sdk/metric"
	"go.opentelemetry.io/otel/sdk/resource"
	sdktrace "go.opentelemetry.io/otel/sdk/trace"
	semconv "go.opentelemetry.io/otel/semconv/v1.26.0"
	"go.opentelemetry.io/otel/trace"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
	"sync"
)

type TraceData struct {
	Ctx  context.Context
	Span trace.Span
}

type Telemetry struct {
	Tracer                    trace.Tracer
	Meter                     metric.Meter
	TelemetryDataByMerkleRoot map[[32]byte]TraceData
	dataMutex                 *sync.Mutex
}

func NewTelemetry(serviceName string, ipPortAddress string, logger logging.Logger) Telemetry {
	ctx := context.Background()

	conn, err := initConn()
	if err != nil {
		logger.Fatal("err", err)
	}

	res, err := resource.New(ctx,
		resource.WithAttributes(
			// The service name used to display traces in backends
			semconv.ServiceNameKey.String(serviceName),
		),
	)
	if err != nil {
		logger.Fatal("err", err)
	}

	_, err = initTracerProvider(ctx, res, conn)
	if err != nil {
		logger.Fatal("err", err)
	}

	_, err = initMeterProvider(ctx, res, conn)
	if err != nil {
		logger.Fatal("err", err)
	}

	name := "" // Using default name for provider
	tracer := otel.Tracer(name)
	meter := otel.Meter(name)

	return Telemetry{
		Tracer:                    tracer,
		Meter:                     meter,
		TelemetryDataByMerkleRoot: make(map[[32]byte]TraceData),
		dataMutex:                 &sync.Mutex{},
	}
}

// Telemetry functions

// InitNewTrace Init a new trace for the given batchMerkleRoot
// User must make sure to call FinishTrace()
func (t *Telemetry) InitNewTrace(batchMerkleRoot [32]byte) {
	merkleRootString := hex.EncodeToString(batchMerkleRoot[:])
	ctx, span := t.Tracer.Start(
		context.Background(),
		fmt.Sprintf("Response for 0x%s", merkleRootString),
		trace.WithAttributes(attribute.String("merkle_root", fmt.Sprintf("0x%s", merkleRootString))),
	)
	t.dataMutex.Lock()
	defer t.dataMutex.Unlock()
	t.TelemetryDataByMerkleRoot[batchMerkleRoot] = TraceData{
		Ctx:  ctx,
		Span: span,
	}
}

// FinishTrace finishes the trace for the given merkle root and frees resources
func (t *Telemetry) FinishTrace(batchMerkleRoot [32]byte) {
	span := t.getSpan(batchMerkleRoot)
	span.End()
	t.dataMutex.Lock()
	defer t.dataMutex.Unlock()
	delete(t.TelemetryDataByMerkleRoot, batchMerkleRoot)
}

// OperatorResponseTrace
// User must call to `defer span.End()` to make sure the span is correctly finished
// For example
// ```
//
//	span := telemetry.OperatorResponseTrace(batchMerkleRoot, operatorId)
//	defer span.End()
//
// ```
func (t *Telemetry) OperatorResponseTrace(batchMerkleRoot [32]byte, operatorId [32]byte) trace.Span {
	ctx := t.getCtx(batchMerkleRoot)
	operatorIdString := hex.EncodeToString(operatorId[:])
	_, span := t.Tracer.Start(
		ctx,
		fmt.Sprintf("Operator ID: 0x%s", operatorIdString),
		trace.WithAttributes(attribute.String("merkle_root", fmt.Sprintf("0x%s", hex.EncodeToString(batchMerkleRoot[:])))),
		trace.WithAttributes(attribute.String("operator_id", fmt.Sprintf("0x%s", operatorIdString))),
	)
	return span
}

// QuorumReachedTrace
// User must call to `defer span.End()` to make sure the span is correctly finished
// For example
// ```
//
//	span := telemetry.QuorumReachedTrace(batchMerkleRoot)
//	defer span.End()
//
// ```
func (t *Telemetry) QuorumReachedTrace(batchMerkleRoot [32]byte) trace.Span {
	ctx := t.getCtx(batchMerkleRoot)
	_, span := t.Tracer.Start(
		ctx,
		fmt.Sprintf("Quorum reached"),
		trace.WithAttributes(attribute.String("merkle_root", fmt.Sprintf("0x%s", hex.EncodeToString(batchMerkleRoot[:])))),
	) // TODO add quorum %
	return span
}

func (t *Telemetry) getCtx(batchMerkleRoot [32]byte) context.Context {
	t.dataMutex.Lock()
	defer t.dataMutex.Unlock()
	return t.TelemetryDataByMerkleRoot[batchMerkleRoot].Ctx
}

func (t *Telemetry) getSpan(batchMerkleRoot [32]byte) trace.Span {
	t.dataMutex.Lock()
	defer t.dataMutex.Unlock()
	return t.TelemetryDataByMerkleRoot[batchMerkleRoot].Span
}

// Initialization functions
// Initialize a gRPC connection to be used by both the tracer and meter
// providers.
func initConn() (*grpc.ClientConn, error) {
	// It connects the OpenTelemetry Collector through local gRPC connection.
	// You may replace `localhost:4317` with your endpoint.
	conn, err := grpc.NewClient("localhost:4317",
		// Note the use of insecure transport here. TLS is recommended in production.
		grpc.WithTransportCredentials(insecure.NewCredentials()),
	)
	if err != nil {
		return nil, fmt.Errorf("failed to create gRPC connection to collector: %w", err)
	}

	return conn, err
}

// Initializes an OTLP exporter, and configures the corresponding trace provider.
func initTracerProvider(ctx context.Context, res *resource.Resource, conn *grpc.ClientConn) (func(context.Context) error, error) {
	// Set up a trace exporter
	traceExporter, err := otlptracegrpc.New(ctx, otlptracegrpc.WithGRPCConn(conn))
	if err != nil {
		return nil, fmt.Errorf("failed to create trace exporter: %w", err)
	}

	// Register the trace exporter with a TracerProvider, using a batch
	// span processor to aggregate spans before export.
	bsp := sdktrace.NewBatchSpanProcessor(traceExporter)
	tracerProvider := sdktrace.NewTracerProvider(
		sdktrace.WithSampler(sdktrace.AlwaysSample()),
		sdktrace.WithResource(res),
		sdktrace.WithSpanProcessor(bsp),
	)
	otel.SetTracerProvider(tracerProvider)

	// Set global propagator to tracecontext (the default is no-op).
	otel.SetTextMapPropagator(propagation.TraceContext{})

	// Shutdown will flush any remaining spans and shut down the exporter.
	return tracerProvider.Shutdown, nil
}

// Initializes an OTLP exporter, and configures the corresponding meter provider.
func initMeterProvider(ctx context.Context, res *resource.Resource, conn *grpc.ClientConn) (func(context.Context) error, error) {
	metricExporter, err := otlpmetricgrpc.New(ctx, otlpmetricgrpc.WithGRPCConn(conn))
	if err != nil {
		return nil, fmt.Errorf("failed to create metrics exporter: %w", err)
	}

	meterProvider := sdkmetric.NewMeterProvider(
		sdkmetric.WithReader(sdkmetric.NewPeriodicReader(metricExporter)),
		sdkmetric.WithResource(res),
	)
	otel.SetMeterProvider(meterProvider)

	return meterProvider.Shutdown, nil
}
