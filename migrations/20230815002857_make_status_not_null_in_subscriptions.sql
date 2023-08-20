-- Add migration script here
BEGIN;
    --Backfill
    UPDATE subscriptions
        SET status = 'confirmed'
        WHERE status IS NULL;
    --Make mandatory
    ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;
COMMIT;