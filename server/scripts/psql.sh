if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

DB_USER="${POSTGRES_USER:=postgres}"
DB_PORT="${POSTGRES_PORT:=5432}"

export PGPASSWORD="${DB_PASSWORD:=password}"

psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "wiki"
