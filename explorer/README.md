# Explorer

## Steps to build

1. Clone [Aligned Layer](https://github.com/yetanotherco/aligned_layer/) inside the `/tmp` directory.
   ```
   cd /tmp/
   git clone https://github.com/yetanotherco/aligned_layer
   ```

3. Head to the `explorer/` folder and run the following command:
   ```
   RPC_URL=
   ENVIRONMENT=
   ALIGNED_CONFIG_FILE=
   PHX_HOST=
   ELIXIR_HOSTNAME=
   PHX_SERVER=true
   DB_NAME=
   DB_USER=
   DB_PASS=
   DB_HOST=
   TRACKER_API_URL=
   SECRET_KEY_BASE=
   KEYFILE_PATH=/home/app/.ssl/key.pem
   CERTFILE_PATH=/home/app/.ssl/cert.pem
   make create_env
   ```
   > The values to be filled can be found inside the documentation.

3. Build the elixir application.
   ```
   make build_explorer
   ```

4. Move the built binary outside `/tmp/` and move it to the actual location.
   ```
   make install_explorer
   ```

5. Set the `CAP_NET_BIND_SERVICE=+eip` to the elixir application to allow elixir to bind port `443`
   ```
   make set_cap
   ```
   > This step must be ran as user `admin`.

7. Enable the systemd service.
   ```
   make run_service
   ```
