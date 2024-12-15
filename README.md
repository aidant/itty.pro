```shell
cargo install sqlx-cli --no-default-features --features sqlite
pnpm i
mkcert -cert-file packages/api/localhost-cert.pem -key-file packages/api/localhost-key.pem "localhost" "127.0.0.1" "::1"

sqlx migrate run --database-url sqlite://./sqlite.db --source packages/api/src/
cargo run --features include_app
```
