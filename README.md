# Postgres MCP Server - RUST
> let's see where this will take us all

### Create a container
using docker
```
docker build --platform linux/amd64 -t rust-mcp-postgres:latest -f dockerfile.postgres_mcp_rust .
```

### Using Docker-Compose
docker-compose up -d

#### Postgres
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