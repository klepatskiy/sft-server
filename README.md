# prjetforfun
docker postgres

use cargo build to generate proto
```shell
cargo build
```

## Migration
use sqlx migrate to create a migration
```shell
sqlx migrate add <migration_name>
```

Execute migration
```shell
sqlx migrate run
```

## start server
```shell
cargo run --package prjetforfun --bin server
```
