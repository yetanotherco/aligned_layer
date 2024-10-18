#!/bin/bash

source .env

mix compile --force #force recompile to get the latest .env values

elixir --sname telemetry -S mix phx.server
