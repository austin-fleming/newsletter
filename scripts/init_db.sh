#! /usr/bin/env bash
set -x
set -eo pipefail

if ![ command -v psql >/dev/null 2>&1 ]; then
    echo "Error: psql is not installed"
    exit 1
fi

if ![ command -v sqlx >/dev/null 2>&1 ]; then
    echo "Error: sqlx is not installed"
    exit 1
fi

DB_USER=${POSTGRES_USER}
DB_PASSWORD="${POSTGRES_PASSWORD}"
DB_NAME="${POSTGRES_DB}"
DB_PORT="${POSTGRES_PORT}"
DB_HOST="${POSTGRES_HOST}"

docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000
    # ^ Increased maximum number of connections for testing purposes

export PGPASSWORD="${DB_PASSWORD}"
until psql -h "${DB_HOST}" -U "${DB_USER}" -d "${DB_NAME}" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done
>&2 echo "Postgres is up and running on port ${DB_PORT} - executing command"


DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL
sqlx database create






# NOTES:
# `chmod +x scripts/init_db.sh` for permissions

# source .env