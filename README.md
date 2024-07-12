# Rust JSON API template
## Main Crates:
 - ### [SQLx](https://docs.rs/sqlx)
SQLx is an async, pure Rust SQL crate featuring compile-time checked queries without a DSL.
- ### [axum](https://docs.rs/axum)
axum is a web application framework that focuses on ergonomics and modularity.

## Additional crates
[tokio](https://docs.rs/tokio) - [serde](https://docs.rs/serde) - [dotenv](https://docs.rs/dotenv) - [bcrypt](https://docs.rs/bcrypt) - [uuid](https://docs.rs/uuid)

## Environment
- ### Setup ENV variables by creating a `.env` at the root of the project:
```
POSTGRES_HOST=<YOUR PG HOST>
POSTGRES_PORT=<YOUR PG PORT>
POSTGRES_USER=<YOUR PG USER>
POSTGRES_PASSWORD=<YOUR PG PASSWORD>
POSTGRES_DB=<YOUR PG DATABASE NAME>

DATABASE_URL=<YOUR PG DATABASE URL>

PGADMIN_DEFAULT_EMAIL=<YOUR PG EMAIL>
PGADMIN_DEFAULT_PASSWORD=<YOUR PG PASSWORD>

SIGNED_JAR_KEY=<SIGNED JAR KEY>
```
- ### Install SQLx CLI
Run `cargo install sqlx-cli` to install it.

## Database
- ### Creating database
Run `sqlx database create` to create a new database using the params given on `.env`.

- ### Adding migrations
Use `sqlx migrate add <name>` to add a new `.down.sql` and `.down.sql` migration files.

- ### Running migrations
To run a your migrations use the command `sqlx migrate run`.

## Models
- Models are located in the `src/models` folder.
- Models have multiple structs that aid on defining the JSON body of a certain request or the return of a query.
- Model structs that are supposed to be used in a JSON body derive: `#[derive(Debug, Deserialize, Serialize)]`
- Model structs that are used on queries derive: `#[derive(Debug, Deserialize, Serialize, Clone)]`

## Handlers
- Handler are located in the `src/handlers` folder.
- Handlers are supposed to just call queries.

## Routes
- Routes are located in the `src/routes` folder.
- Routes should appoint directly to an handler method, for example: `route("/users", post(user::create))`

## Config
- Config are located in the `src/config` folder.
- Contains the configuration for the App's Database, App state and Router

## Utils
- Utils are located in the `src/utils` folder.

## Structs
- Structs are located in the `src/structs` folder.
- Unlike model structs, the structs located here are meant for the internal functions of the app. 

## Features
- This template comes with an user `[model]`, `handler` and `routes` setup.
- User has a simple signed cookie based authentication setup.
