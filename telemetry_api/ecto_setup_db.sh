#!/bin/bash

source .env.dev

mix deps.get

mix compile --force #force recompile to get the latest .env values

mix ecto.create
mix ecto.migrate
