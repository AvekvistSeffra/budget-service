# Budget Service API
A budget service written in Rust using axum and sqlx. 

Uses a Postgres datastore.

Clone the project, set the apt config.yaml for your machine and database, 
compile, then run. 

```sh
cargo run
```

## TODO
- implement `PUT` on all objects
- maybe add auth to protect income and expenses from prying eyes
- add proper logging and configuration for logging
- auto-updating of conversion rates based on currency from some API or something
