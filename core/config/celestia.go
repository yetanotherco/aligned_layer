package config

import (
	"context"
	"crypto/rand"
	"errors"
	sdkutils "github.com/Layr-Labs/eigensdk-go/utils"
	"github.com/celestiaorg/celestia-node/api/rpc/client"
	"github.com/celestiaorg/celestia-node/libs/authtoken"
	"github.com/celestiaorg/celestia-node/libs/keystore"
	nodemod "github.com/celestiaorg/celestia-node/nodebuilder/node"
	"github.com/celestiaorg/celestia-node/share"
	"github.com/cristalhq/jwt"
	"github.com/filecoin-project/go-jsonrpc/auth"
	"github.com/mitchellh/go-homedir"
	"io"
	"log"
	"os"
	"path/filepath"
)

type CelestiaConfig struct {
	Client    *client.Client
	Namespace share.Namespace
}

type CelestiaConfigFromYaml struct {
	Celestia struct {
		Url          string `yaml:"url"`
		KeystorePath string `yaml:"keystore"`
	} `yaml:"celestia"`
}

func NewCelestiaConfig(celestiaConfigFilePath string, permissions []auth.Permission) *CelestiaConfig {
	if _, err := os.Stat(celestiaConfigFilePath); errors.Is(err, os.ErrNotExist) {
		log.Fatal("Setup celestia config file does not exist")
	}

	var celestiaConfigFromYaml CelestiaConfigFromYaml
	err := sdkutils.ReadYamlConfig(celestiaConfigFilePath, &celestiaConfigFromYaml)
	if err != nil {
		log.Fatal("Error reading celestia config: ", err)
	}

	if celestiaConfigFromYaml.Celestia.Url == "" {
		log.Fatal("Celestia url is empty")
	}

	ks, err := newKeystore(celestiaConfigFromYaml.Celestia.KeystorePath)
	if err != nil {
		log.Fatal(err)
	}

	key, err := ks.Get(nodemod.SecretName)
	if err != nil {
		if !errors.Is(err, keystore.ErrNotFound) {
			log.Fatal(err)
		}
		key, err = generateNewKey(ks)
		if err != nil {
			log.Fatal(err)
		}
	}

	token, err := buildJWTToken(key.Body, permissions)
	if err != nil {
		log.Fatal(err)
	}

	c, err := client.NewClient(context.Background(), celestiaConfigFromYaml.Celestia.Url, token)
	if err != nil {
		log.Fatal(err)
	}

	ns, err := share.NewBlobNamespaceV0([]byte("Aligned"))
	if err != nil {
		log.Fatal(err)
	}

	return &CelestiaConfig{
		Client:    c,
		Namespace: ns,
	}

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
