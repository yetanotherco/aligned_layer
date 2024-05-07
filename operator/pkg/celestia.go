package operator

import (
	"context"
	"crypto/rand"
	"errors"
	"github.com/celestiaorg/celestia-node/api/rpc/client"
	"github.com/celestiaorg/celestia-node/api/rpc/perms"
	"github.com/celestiaorg/celestia-node/blob"
	"github.com/celestiaorg/celestia-node/libs/authtoken"
	"github.com/celestiaorg/celestia-node/libs/keystore"
	nodemod "github.com/celestiaorg/celestia-node/nodebuilder/node"
	"github.com/celestiaorg/celestia-node/share"
	"github.com/cristalhq/jwt"
	"github.com/filecoin-project/go-jsonrpc/auth"
	"github.com/mitchellh/go-homedir"
	"io"
	"path/filepath"
)

func (o *Operator) getProofFromCelestia(height uint64, namespace share.Namespace, commitment blob.Commitment) ([]byte, error) {

	// TODO: Remove hardcoded path
	ks, err := newKeystore("~/.celestia-light-arabica-11")
	if err != nil {
		o.Logger.Error("failed to create keystore", "err", err)
		return nil, err
	}

	key, err := ks.Get(nodemod.SecretName)
	if err != nil {
		if !errors.Is(err, keystore.ErrNotFound) {
			o.Logger.Error("failed to get key from keystore", "err", err)
			return nil, err
		}
		key, err = generateNewKey(ks)
		if err != nil {
			o.Logger.Error("failed to generate new key", "err", err)
			return nil, err
		}
	}

	token, err := buildJWTToken(key.Body, perms.ReadWritePerms)
	if err != nil {
		o.Logger.Error("failed to build JWT token", "err", err)
		return nil, err
	}

	// TODO: Remove hardcoded address
	cli, err := client.NewClient(context.Background(), "http://localhost:26658", token)
	if err != nil {
		o.Logger.Error("failed to create client", "err", err)
		return nil, err
	}

	blob, err := cli.Blob.Get(context.Background(), height, namespace, commitment)
	if err != nil {
		return nil, err
	}

	return blob.Data, nil
}

func newKeystore(path string) (keystore.Keystore, error) {
	expanded, err := homedir.Expand(filepath.Clean(path))
	if err != nil {
		return nil, err
	}
	return keystore.NewFSKeystore(filepath.Join(expanded, "keys"), nil)
}

func buildJWTToken(body []byte, permissions []auth.Permission) (string, error) {
	signer, err := jwt.NewHS256(body)
	if err != nil {
		return "", err
	}
	return authtoken.NewSignedJWT(signer, permissions)
}

func generateNewKey(ks keystore.Keystore) (keystore.PrivKey, error) {
	sk, err := io.ReadAll(io.LimitReader(rand.Reader, 32))
	if err != nil {
		return keystore.PrivKey{}, err
	}
	// save key
	key := keystore.PrivKey{Body: sk}
	err = ks.Put(nodemod.SecretName, key)
	if err != nil {
		return keystore.PrivKey{}, err
	}
	return key, nil
}
