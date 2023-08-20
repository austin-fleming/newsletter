DB_USER=${POSTGRES_USER}
DB_PASSWORD=${POSTGRES_PASSWORD}
DB_NAME=${POSTGRES_DB}
DB_PORT=${POSTGRES_PORT}
DB_HOST=${POSTGRES_HOST}

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL

sqlx migrate add create_subscribers_table
sqlx migrate add add_status_subscriptions;

# TODO: Add the following to the migration file:
# Congratulations on creating your first migration!

# Did you know you can embed your migrations in your application binary?
# On startup, after creating your database connection or pool, add:

# sqlx::migrate!().run(<&your_pool OR &mut your_connection>).await?;

# Note that the compiler won't pick up new migrations if no Rust source files have changed.
# You can create a Cargo build script to work around this with `sqlx migrate build-script`.

# See: https://docs.rs/sqlx/0.5/sqlx/macro.migrate.html


# Migrating to a target
# `DATABASE_URL=<connection-string> sqlx migrate run`
#      -- "trusted sources" needs to be turned off in Digital Ocean to do this from local machine.