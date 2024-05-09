package pkg_test

import (
	"github.com/stretchr/testify/assert"
	"github.com/yetanotherco/aligned_layer/task_sender/pkg"
	"testing"
)

func TestSplitIntoChunks(t *testing.T) {
	proof := []byte("proof")
	chunkSize := uint64(2)

	chunks := pkg.SplitIntoChunks(proof, chunkSize)
	assert.Equal(t, 3, len(chunks))
	assert.Equal(t, []byte("pr"), chunks[0])
	assert.Equal(t, []byte("oo"), chunks[1])
	assert.Equal(t, []byte("f"), chunks[2])
}

func TestSplitIntoChunksDivisible(t *testing.T) {
	proof := []byte("even")
	chunkSize := uint64(2)

	chunks := pkg.SplitIntoChunks(proof, chunkSize)
	assert.Equal(t, 2, len(chunks))
	assert.Equal(t, []byte("ev"), chunks[0])
	assert.Equal(t, []byte("en"), chunks[1])
}

func TestSplitIntoChunksLessThanOne(t *testing.T) {
	proof := []byte("h")
	chunkSize := uint64(2)

	chunks := pkg.SplitIntoChunks(proof, chunkSize)
	assert.Equal(t, 1, len(chunks))
	assert.Equal(t, []byte("h"), chunks[0])
}
