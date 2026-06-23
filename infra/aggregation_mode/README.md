# Aggregation Mode Setup

## Overview

The aggregation mode runs on a Paperspace GPU server. To avoid paying for the GPU
24/7, the server is kept powered off and is only started once a day:

1. The [`Start Aggregation Mode Server`](../../.github/workflows/aggregation_mode.yml)
   GitHub Actions workflow runs daily at 15:00 UTC (12:00 GMT-3) and starts the
   Paperspace machine via the Paperspace API.
2. On boot, `aggregation_mode.service` runs automatically and executes
   [`run.sh`](run.sh), which runs the SP1 and Risc0 aggregations.
3. When the aggregations finish, `run.sh` powers the machine off again
   (`sudo shutdown -h now`), so Paperspace stops billing it.

The workflow needs:

- `PAPERSPACE_API_KEY` repository **secret** — a Paperspace API key.
- `PAPERSPACE_MACHINE_ID` repository **secret** — the id of the GPU machine.

## Setup on Server with GPU

To setup the server with GPU, follow the steps in [aggregation_mode.sh](aggregation_mode.sh).

After running all the steps, `aggregation_mode.service` is enabled to run on every
boot. There is no timer anymore — the daily schedule is driven by the GitHub Actions
workflow described above.

## Check Service Status

To check the status of the service, run:

```bash
systemctl status aggregation_mode.service --user
```

## Start Service manually

If you need to start the service manually (the machine is already on), run:

```bash
systemctl start aggregation_mode.service --user
```

## Check Logs

To check the logs of the service, run:

```bash
journalctl -xfeu aggregation_mode.service --user
```

Note: You can add `-n <n_of_lines>` to limit the number of lines to show.
