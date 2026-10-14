-- One-off script for the SP1 v6 cutover.
--
-- Marks the SP1 tasks that are still queued and were submitted before the cutover as failed.
-- Those tasks hold SP1 v5 proofs, which the v6 aggregator can not deserialize nor aggregate.
-- Run it after the v6 gateway is deployed, with cutover_ts set to the time that deploy finished.
--
-- Usage:
--   psql "$DATABASE_URL" -v cutover_ts='2026-10-20 15:00:00+00' -f mark_sp1_v5_tasks_failed.sql

\set ON_ERROR_STOP on

BEGIN;

-- proving_system_id = 1 is SP1 (see AggregationModeProvingSystem)
UPDATE tasks
SET status = 'failed', status_updated_at = now()
WHERE proving_system_id = 1
  AND status IN ('pending', 'processing')
  AND inserted_at < :'cutover_ts';

COMMIT;
