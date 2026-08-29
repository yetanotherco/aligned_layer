.PHONY: build_explorer install_explorer set_cap create_env

ifneq (,$(wildcard ./.env))
    include .env
    export
endif

create_env:
	truncate -s0 /tmp/aligned_layer/explorer/.env
	echo "RPC_URL=${RPC_URL}" >> /tmp/aligned_layer/explorer/.env
	echo "ENVIRONMENT=${ENVIRONMENT}" >> /tmp/aligned_layer/explorer/.env
	echo "ALIGNED_CONFIG_FILE=${ALIGNED_CONFIG_FILE}" >> /tmp/aligned_layer/explorer/.env
	echo "PHX_HOST=${PHX_HOST}" >> /tmp/aligned_layer/explorer/.env
	echo "ELIXIR_HOSTNAME=${ELIXIR_HOSTNAME}" >> /tmp/aligned_layer/explorer/.env
	echo "PHX_SERVER=true" >> /tmp/aligned_layer/explorer/.env
	echo "DB_NAME=${DB_NAME}" >> /tmp/aligned_layer/explorer/.env
	echo "DB_USER=${DB_USER}" >> /tmp/aligned_layer/explorer/.env
	echo "DB_PASS=${DB_PASS}" >> /tmp/aligned_layer/explorer/.env
	echo "DB_HOST=${DB_HOST}" >> /tmp/aligned_layer/explorer/.env
	echo "TRACKER_API_URL=${TRACKER_API_URL}" >> /tmp/aligned_layer/explorer/.env
	echo "SECRET_KEY_BASE=${SECRET_KEY_BASE}" >> /tmp/aligned_layer/explorer/.env
	echo "KEYFILE_PATH=/home/app/.ssl/key.pem" >> /tmp/aligned_layer/explorer/.env
	echo "CERTFILE_PATH=/home/app/.ssl/cert.pem" >> /tmp/aligned_layer/explorer/.env

# on /tmp/aligned_layer/explorer/ as app
build_explorer: export MIX_ENV=prod
build_explorer:
	mix local.hex --force
	mix local.rebar --force
	mix deps.get --only $(MIX_ENV)
	echo $(ENVIRONMENT)
	mix compile
	pnpm --prefix=assets/ install
	mix phx.digest
	mix assets.deploy
	mix release --overwrite

# on /tmp/aligned_layer/explorer/ as app
install_explorer: 
	mkdir -p /home/app/repos/explorer/
	mv /tmp/aligned_layer/ /home/app/repos/explorer/
	
# on /home/app/repos/explorer/aligned_layer/explorer/ as admin
set_cap:
	sudo setcap CAP_NET_BIND_SERVICE=+eip /home/app/repos/explorer/aligned_layer/explorer/_build/prod/rel/explorer/erts-14.2.1/bin/beam.smp

# on /home/app/repos/explorer/aligned_layer/explorer/ as app
run_service:
	mkdir -p /home/app/config/
	cp /home/app/repos/explorer/aligned_layer/explorer/.env /home/app/config/.env.explorer
	systemctl --user daemon-reload
	systemctl --user enable --now explorer
