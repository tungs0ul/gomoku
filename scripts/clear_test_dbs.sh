#!/bin/bash

# Database connection details
PGUSER="${SUPERUSER:=postgres}"
PGPASSWORD="${SUPERUSER_PWD:=password}"
PGHOST="${DB_HOST:=localhost}"
PGPORT="${DB_PORT:=5432}"

# Export the password for non-interactive use
export PGPASSWORD="$PGPASSWORD"

# Connect to PostgreSQL and fetch databases starting with 'test_'
databases=$(psql -U "$PGUSER" -h "$PGHOST" -p "$PGPORT" -d postgres -t -c "SELECT datname FROM pg_database WHERE datname LIKE 'test_%';")

# Loop through the databases and drop them
for db in $databases; do
    if [ -n "$db" ]; then
        echo "Dropping database: $db"
        psql -U "$PGUSER" -h "$PGHOST" -p "$PGPORT" -d postgres -c "DROP DATABASE \"$db\";"
    fi
done

# Unset the password for security
unset PGPASSWORD

echo "All databases starting with 'test_' have been dropped."
