# Batcher Watchdog

The Batcher Watchdog checks a prometheus metric and restart the batcher as needed

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
*/20 * * * * /path/to/watchdog/batcher_watchdog.sh /path/to/config/.env >> /path/to/logs/folder/batcher_watchdog.log 2>&1
```

You can check logs in the specified file, for example:

```
Tue Oct 15 08:00:01 UTC 2024: tasks created in the last 20m: "25"
Tue Oct 15 08:20:01 UTC 2024: tasks created in the last 20m: "2"
Tue Oct 15 08:40:01 UTC 2024: tasks created in the last 20m: "0"
Tue Oct 15 08:40:01 UTC 2024: restarting batcher
Tue Oct 15 08:40:01 UTC 2024: batcher restarted
```
