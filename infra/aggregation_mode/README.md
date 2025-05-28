# Aggregation Mode Setup

## Setup on Server with GPU

To setup the server with GPU, follow the steps in [aggregation_mode.sh](aggregation_mode.sh).

After running all the steps, `aggregation_mode.timer` will execute every 24hs the `aggregation_mode.service`

## Check Service Status

To check the status of the timer, run:

```bash
systemctl status aggregation_mode.timer --user
```

To check the status of the service, run:

```bash
systemctl status aggregation_mode.service --user
```

## Start Service manually

If you need to start the service manually, without waiting for the timer, run:

```bash
systemctl start aggregation_mode.service --user
```

## Check Logs

To check the logs of the service, run:

```bash
journalctl -xfeu aggregation_mode.service --user 
```

Note: You can add `-n <n_of_lines>` to limit the number of lines to show.
