# gui-api

## Setup
### Native
1. Install the following:
- [rust](https://www.rust-lang.org/learn/get-started)
- [diesel cli](https://diesel.rs/guides/getting-started)
- [docker](https://docs.docker.com/get-docker/)

2. Setup the db
```sh
cp example.env .env
docker run --name postgres -e POSTGRES_PASSWORD=toor123 -p 5432:5432 -d postgres
# Wait for a few seconds for the db to start
diesel setup
```
- To start the docker db again
```sh
docker start postgress
```

3. Run the server
- Debug: `cargo run`
- Release: `cargo run --release`
- arguments: `cargo run [OPTIONS] -- --help`
- NOTES:
  - Values in databse-url must be url encoded
  - Release build does not read the `.env` file
  - Debug build has a permissive CORS layer RELEASE does not

### Docker
```bash
#docker run ewoutvdb/gui-api --database-url 'postgres://<user-name>:<password>@<server>:<port>/<database>'
docker run -p 3000:3000 ewoutvdb/ai-meter --database-url 'postgres://postgres:toor123@localhost:5432/meter' --addr 0.0.0.0:3000 --log-level debug
```
