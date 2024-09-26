#!/bin/bash

source .env.dev

mix compile --force #force recompile to get the latest .env values

mix phx.server
