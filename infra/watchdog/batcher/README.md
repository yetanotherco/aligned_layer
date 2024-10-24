# Batcher Watchdog

The Batcher Watchdog checks a prometheus metric and restart the Batcher as needed.

The metric is the quantity of batches sent in the last N minutes, defined in the PROMETHEUS_INTERVAL variable. Lets call this metric `sent_batches`.

Since we are sending proofs constantly, the ideal behaviour is the creation of a task every 3 Ethereum blocks (~36 secs). So, if the `sent_batches` metrics is 0 it means there is a problem in the Batcher, for example a transaction is stuck in Ethereum and the Batcher is locked waiting for the transaction. If this happens, the Watchdog restarts the Batcher.

## Configuration

You need to create a .env file with the following variables

```
PROMETHEUS_URL=<ip>:<port>
SYSTEMD_SERVICE=batcher
PROMETHEUS_COUNTER=sent_batches
PROMETHEUS_BOT=batcher
PROMETHEUS_INTERVAL=20m
SLACK_WEBHOOK_URL=<>
```

There is a `.env.example` file in this directory.

## Run with Crontab

Open the Crontab configuration with `crontab -e` and add the following line:

```
*/10 * * * * /path/to/watchdog/batcher_watchdog.sh /path/to/config/.env >> /path/to/logs/folder/batcher_watchdog.log 2>&1
```

The cron interval has to be the half of PROMETHEUS_INTERVAL (PROMETHEUS_INTERVAL/2).

You can check logs in the specified file, for example:

```
Tue Oct 15 08:00:01 UTC 2024: tasks created in the last 20m: "25"
Tue Oct 15 08:20:01 UTC 2024: tasks created in the last 20m: "2"
Tue Oct 15 08:40:01 UTC 2024: tasks created in the last 20m: "0"
Tue Oct 15 08:40:01 UTC 2024: restarting batcher
Tue Oct 15 08:40:01 UTC 2024: batcher restarted
```
