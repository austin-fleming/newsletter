## SQLX
- To create a migration: ```sqlx migrate add <title>``` then populate with SQL.
- Run migration with ```sqlx migrate run```

### Initial Setup Process
- Create db with ```./scripts/db_init.sh```
- Create migrations file with ```sqlx migrate add <migration-name>``` then populate with SQL
- Run migration with ```sqlx migrate run```

### Process
- Creating database
    - On local
        - Create db with ```./scripts/db_init.sh```
        - Create migrations file with ```sqlx migrate add <migration-name>``` then populate with SQL
        - Run migration with ```sqlx migrate run```
        - Test queries work.
    - Deploy live database and app

- Updating database
    - Step 01: Adding fields as temporarily optional
        - On local
            - Begin with an intermediary migration that creates the new fields, but doesn't make them NOT NULL. Create migrations file with ```sqlx migrate add <migration-name>``` then populate with SQL.
            - Update queries so that they handle any new requests to fill fields that will later be set NOT NULL.
            - Test it works
        - Update remote db and app
        - NOTE: we need to make sure that the API will start populating this field. All entries at this point forward should be populating the new field.
    - Step 02: Backfilling Fields
        - On local
            - Create a migration that updates all existing fields with a value for any fields that will become NOT NULL
            - run migration
            - test it works
        - Update remote db and app
        - NOTE: Step 01 ensured all new entries are populated. This step ensures old entries before step 01 are now populated as well.
    - Step 03: Setting fields as required
        - On local
            - Create a migration that sets fields to NOT NULL
            - run migration
            - test it works
        - Update remote db and app
    - STEP 2b: combining 02 + 03
        - Step 2 & 3 can be combined if the migration script handles both backfilling rows and updating the column to NOT NULL. The entire migration in this case must be wrapped in an atomic transaction to ensure any failures mean a reversion. For example:```
        BEGIN;
            --Backfill
            UPDATE subscriptions
                SET status = 'confirmed'
                WHERE status IS NULL;
            --Make mandatory
            ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;
        COMMIT;
        ```