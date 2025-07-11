# Postgres MCP Server - RUST
> let's see where this will take us all

### Create a container
using docker
```
docker build --platform linux/amd64 -t rust-mcp-postgres:latest -f dockerfile.postgres_mcp_rust .
```

### Using Docker-Compose
Run this command (you need the `.env` file - see below):
```text
docker-compose --env-file=.env up
```
This here is what should be outputted:
```text
Starting rust-mcp-mysql_psql_1                ... done
Creating rust-mcp-mysql_postgres-mcp-server_1 ... done
Attaching to rust-mcp-mysql_psql_1, rust-mcp-mysql_postgres-mcp-server_1
psql_1                 | 
psql_1                 | PostgreSQL Database directory appears to contain a database; Skipping initialization
psql_1                 | 
psql_1                 | 2025-07-08 13:58:15.283 UTC [1] LOG:  starting PostgreSQL 17.5 (Debian 17.5-1.pgdg120+1) on x86_64-pc-linux-gnu, compiled by gcc (Debian 12.2.0-14) 12.2.0, 64-bit
psql_1                 | 2025-07-08 13:58:15.283 UTC [1] LOG:  listening on IPv4 address "0.0.0.0", port 5432
psql_1                 | 2025-07-08 13:58:15.283 UTC [1] LOG:  listening on IPv6 address "::", port 5432
psql_1                 | 2025-07-08 13:58:15.289 UTC [1] LOG:  listening on Unix socket "/var/run/postgresql/.s.PGSQL.5432"
psql_1                 | 2025-07-08 13:58:15.297 UTC [29] LOG:  database system was shut down at 2025-07-08 13:28:06 UTC
psql_1                 | 2025-07-08 13:58:15.306 UTC [1] LOG:  database system is ready to accept connections
postgres-mcp-server_1  | 2025-07-08T13:58:15.311910Z  INFO rust_mcp_mysql: sse server started!
```
The above `docker-compose` command can be started with `-d` for detached mode.  

#### Create .env
Create a `.env` file with that database, user, password, host and port.
```
POSTGRES_HOST=localhost
POSTGRES_PORT=5432
POSTGRES_USER=postgres
POSTGRES_PASSWORD=mysecretpassword
POSTGRES_DATABASE=postgres
```

#### Postgres w. pgvector
**Be Aware** you need to enable pgvector to get the full potential of this here demo.  
Connect to the running container
```text
docker exec -it <name of container | rust-mcp-postgres_psql> bash
```
start psql
```text
psql -U postgres -d postgres
```
Then execute the following command to enable the extension.
```sql
CREATE EXTENSION IF NOT EXISTS vector;
```

###### Errors and hiccups
If you encounter the following error:
```text
psql_1  | 2025-07-08 12:20:15.913 UTC [49] FATAL:  no pg_hba.conf entry for host "172.28.0.1", user "postgres", database "postgres", no encryption
```
Access the running container:
```text
docker exec -it rust-mcp-mysql_psql_1 bash
```
Then run the following command, to grand access to all:
```text
echo -e "host    all             all             all             	trust">>/var/lib/postgresql/data/pg_hba.conf
```
After the above has been run we need to restart the docker-compose.

