# Rust/Actix Framework

[![Build Status](https://travis-ci.com/ddimaria/rust-actix-framework.svg?branch=master)](https://travis-ci.com/ddimaria/rust-actix-framework)

A web framework built upon Actix Web 3.x using the Rust language.

To view the frontend companion, check out [rust-actix-framework-front](https://github.com/ddimaria/rust-actix-framework-front).

## Motivation

Actix Web is a fast, powerful web framework for building web applications in Rust.
This project aims to create ergonomic abstractions comparable to frameworks in
other languages while attempting to maintain the performance benefits of Rust and Actix.

## Features

- Actix 3.x HTTP Server
- Multi-Database Support (CockroachDB, Postgres, MySQL, Sqlite)
- JWT Support
- Async Caching Layer with a Simple API
- Public and Secure Static File Service
- Diesel Database Operations are Non-Blocking
- Filesystem Organized for Scale
- .env for Local Development
- Integrated Application State with a Simple API
- Lazy Static Config struct
- Built-in Healthcheck (includes cargo version info)
- Listeners configured for TDD
- Custom Errors and HTTP Payload/Json Validation
- Secure Argon2i Password Hashing
- CORS Support
- Paginated Results
- Unit and Integration Tests
- Test Coverage Reports
- Dockerfile for Running the Server in a Container
- TravisCI Integration

## Featured Packages

- `Argon2i`: Argon2i Password Hasning
- `actix-cors`: CORS Support
- `actix-identity`: User Authentication
- `actix-redis` and `redis-async`: Async Caching Layer
- `actix-web`: Actix Web Server
- `derive_more`: Error Formatting
- `diesel`: ORM that Operates on Several Databases
- `dotenv`: Configuration Loader (.env)
- `envy`: Deserializes Environment Variables into a Config Struct
- `jsonwebtoken`: JWT encoding/decoding
- `kcov`: Coverage Analysis
- `listenfd`: Listens for Filesystem Changes
- `rayon`: Parallelize
- `r2d2`: Database Connection Pooling
- `validator`: Validates incoming Json

# Table of Contents

- [Quick Installation](#quick-installation)
- [Installation](#installation)
- [Running the Server](#running-the-server)
- [Autoreloading](#autoreloading)
- [Tests](#tests)
  - [Running Tests](#running-tests)
  - [Test Covearage](#test-covearage)
- [Docker](#docker)
  - [Docker Compose](#docker-compose)
- [Generating documentation](#generating-documentation)
- [The #[timestamps] proc macro](#the-timestamps-proc-macro)
- [The paginate! declaritive macro](#the-paginate-declaritive-macro)
- [Public Static Files](#public-static-files)
- [Secure Static Files](#secure-static-files)
- [Application State](#application-state)
  - [Helper Functions](#helper-functions)
- [Application Cache](#application-cache)
  - [Helper Functions](#helper-functions)
- [Non-Blocking Diesel Database Operations](#non-blocking-diesel-database-operations)
- [Endpoints](#endpoints)
  - [Healthcheck](#healthcheck)
  - [Login](#login)
  - [Logout](#logout)
  - [Get All Users](#get-all-users)
  - [Get a User](#get-a-user)
  - [Create a User](#create-a-user)
  - [Update a User](#update-a-user)
  - [Delete a User](#delete-a-user)
- [License](#license)

## Quick Installation

You can skip the first portion and jump ahead to the `Diesel CLI` section of this setup by copying the skeleton code in the `/examples` folder.

## Installation

First, create a new project:

```shell
cargo new rest_server --bin
```

Next, cd into the `rest_server` folder and add the following to Cargo.toml:

```toml
[package]
name = "rest_server"
version = "0.1.0"
authors = ["YOUR NAME <yourname@yourdomain.com>"]
edition = "2018"

[dependencies]
actix_framework = "0.2.0"
actix-cors = "0.2.0"
actix-rt = "1"
actix-web = "3"
dotenv = "0.14"
env_logger = "0.6"
listenfd = "0.3"


[features]
cockroach = []
mysql = []
postgres = []
sqlite = []
default = ["mysql"]
```

With that setup in place, you can add in the server code in `/src/main.rs`:

```rust
use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use listenfd::ListenFd;
use actix_framework::auth::get_identity_service;
use actix_framework::cache::add_cache;
use actix_framework::config::CONFIG;
use actix_framework::database::add_pool;
use actix_framework::routes::routes;
use actix_framework::state::new_state;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    // Create the application state
    // String is used here, but it can be anything
    // Invoke in hanlders using data: AppState<'_, String>
    let data = new_state::<String>();

    // Initialize the file system listener
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            // Add the default logger
            .wrap(Logger::default())

            // Accept all CORS
            // For more options, see https://docs.rs/actix-cors
            .wrap(Cors::new().supports_credentials().finish())

            // Adds Identity Service for use in the Actix Data Extractor
            // In a handler, add "id: Identity" param for auto extraction
            .wrap(get_identity_service())

            // Adds Application State for use in the Actix Data Extractor
            // In a handler, add "data: AppState<'_, String>" param for auto extraction
            .app_data(data.clone())

            // Adds the Redis Cache for use in the Actix Data Extractor
            // In a handler, add "cache: Cache" param for auto extraction
            .configure(add_cache)

            // Adds a Database Pool for use in the Actix Data Extractor
            // In a handler, add "pool: Data<PoolType>" param for auto extraction
            .configure(add_pool)

            // Pull in default framework defaults
            // This can be removed if they're not needed
            .configure(routes)
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind(&CONFIG.server)?
    };

    server.run().await
}

```

Create an .env file at the root of your project:

```shell
touch .env
```

Now add environment values for local development:

```ini
AUTH_SALT=CHANGEME
DATABASE=mysql
DATABASE_URL=mysql://root:root@0.0.0.0:13306/rust-actix-framework
JWT_EXPIRATION=24
JWT_KEY=4125442A472D4B614E645267556B58703273357638792F423F4528482B4D6251
REDIS_URL=127.0.0.1:6379
RUST_BACKTRACE=0
RUST_LOG="actix_framework=info,actix_web=info,actix_server=info,actix_redis=trace"
SERVER=127.0.0.1:3000
SESSION_KEY=4125442A472D4B614E645267556B58703273357638792F423F4528482B4D6251
SESSION_NAME=auth
SESSION_SECURE=false
SESSION_TIMEOUT=20
```

**IMPORTANT:** Change .env values for your setup, paying special attention to the salt and various keys.

After you set the `DATABASE` value in .env, you'll need it to match the `default` value in the `features` section in `Cargo.toml` with the `DATABASE` value in .env:

```toml
[features]
cockroach = []
mysql = []
postgres = []
sqlite = []
default = ["mysql"]
```

_note:_ Only supply a SINGLE database in the `default` array.

Next, you'll need to install the Diesel CLI:

```shell
cargo install diesel_cli
```

If you run into errors, see http://diesel.rs/guides/getting-started/

After you've created a blank database, run the migrations via the Diesel CLI:

```shell
diesel migration run
```

## Running the Server

To startup the server:

```shell
cargo run
```

## Autoreloading

To startup the server and autoreload on code changes:

```shell
systemfd --no-pid -s http::3000 -- cargo watch -x run
```

## Tests

Integration tests are in the `/src/tests` folder. There are helper functions
to make testing the API straightforward. For example, if we want to test the
`GET /api/v1/user` route:

```rust
  use crate::tests::helpers::tests::assert_get;

  #[test]
  async fn test_get_users() {
      assert_get("/api/v1/user").await;
  }
```

Using the Actix test server, the request is sent and the response is asserted
for a successful response:

`assert!(response.status().is_success());`

Similarly, to test a POST route:

```rust
use crate::handlers::user::CreateUserRequest;
use crate::tests::helpers::tests::assert_post;

#[test]
async fn test_create_user() {
    let params = CreateUserRequest {
        first_name: "Satoshi".into(),
        last_name: "Nakamoto".into(),
        email: "satoshi@nakamotoinstitute.org".into(),
    };
    assert_post("/api/v1/user", params).await;
}
```

### Running Tests

To run all of the tests:

```shell
cargo test
```

### Test Covearage

I created a repo on DockerHub that I'll update with each Rust version
(starting at 1.37), whose tags will match the Rust version.

In the root of the project:

```shell
docker run -it --rm --security-opt seccomp=unconfined --volume "${PWD}":/volume --workdir /volume ddimaria/rust-kcov:1.37 --exclude-pattern=/.cargo,/usr/lib,/src/main.rs,src/server.rs
```

_note: coverage takes a long time to run (up to 30 mins)._

You can view the HTML output of the report at `target/cov/index.html`

## Docker

To build a Docker image of the application:

```shell
docker build -t actix_framework .
```

Once the image is built, you can run the container in port 3000:

```shell
docker run -it --rm --env-file=.env.docker -p 3000:3000 --name actix_framework actix_framework
```

### Docker Compose

To run dependencies for this application, simply invoke docker-compose:

```shell
docker-compose up
```

_Currently, only MySQL is in there, but more to come_

## Generating documentation

```shell
cargo doc --no-deps --open
```

## The #[timestamps] proc macro

The `#[timestamps]` macro will automatically append the following fields to a model struct:

```rust
pub created_by: String,
pub created_at: NaiveDateTime,
pub updated_by: String,
pub updated_at: NaiveDateTime,
```

Example:

```rust
use chrono::NaiveDateTime;
use proc_macro::timestamps;

#[timestamps]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}
```

This will expand to:

```rust
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_by: String,
    pub created_at: NaiveDateTime,
    pub updated_by: String,
    pub updated_at: NaiveDateTime,
}
```

## The paginate! declaritive macro

The `paginate!` macro removes boilerplate for paginating a model.

```rust
macro_rules! paginate {
    ($pool:expr, $model:ident, $model_type:ident, $params:ident, $response_type:ident, $base:ident) => {{
        let conn = $pool.get()?;
        let total = $model.select(count_star()).first(&conn)?;
        let pagination = get_pagination($params.page, $params.per_page, total);
        let paginated: $response_type = $model
            .limit(pagination.per_page)
            .offset(pagination.offset)
            .load::<$model_type>(&conn)?
            .into();

        Ok(paginate::<$response_type>(pagination, paginated, $base)?)
    }};
}
```

Below is an example of using the macro in the user model:

```rust
pub fn get_all(
    pool: &PoolType,
    params: PaginationRequest,
    base: String,
) -> Result<PaginationResponse<UsersResponse>, ApiError> {
    use crate::schema::users::dsl::users;

    crate::paginate!(pool, users, User, params, UsersResponse, base)
}
```

## Public Static Files

Static files are served up from the `/static` folder.
Directory listing is turned off.
Index files are supported (`index.html`).

Example:

```shell
curl -X GET http://127.0.0.1:3000/test.html
```

## Secure Static Files

To serve static files to authenticated users only, place them in the `/static-secure` folder.
These files are referenced using the root-level `/secure` path.

Example:

```shell
curl -X GET http://127.0.0.1:3000/secure/test.html
```

## Application State

A shared, mutable hashmap is automatically added to the server. To invoke this data in a handler, simply add `data: AppState<'_, String>` to the handler's signature.

### Helper Functions

#### get\<T\>(data: AppState\<T\>, key: &str) -> Option\<T\>

Retrieves a copy of the entry in application state by key.

Example:

```rust
use crate::state::get;

pub async fn handle(data: AppState<'_, String>) -> impl Responder {
  let key = "SOME_KEY";
  let value = get(data, key);
  assert_eq!(value, Some("123".to_string()));
}
```

#### set\<T\>(data: AppState\<T\>, key: &str, value: T) -> Option\<T\>

Inserts or updates an entry in application state.

Example:

```rust
use crate::state::set;

pub async fn handle(data: AppState<'_, String>) -> impl Responder {
  let key = "SOME_KEY";
  let value = set(data, key, "123".into());
  assert_eq!(value, None)); // if this is an insert
  assert_eq!(value, Some("123".to_string())); // if this is an update
}
```

#### delete\<T\>(data: AppState\<T\>, key: &str) -> Option\<T\>

Deletes an entry in application state by key.

Example:

```rust
use crate::state::get;

pub async fn handle(data: AppState<'_, String>) -> impl Responder {
  let key = "SOME_KEY";
  let value = delete(data, key);
  assert_eq!(value, None);
}
```

## Application Cache

Asynchronous access to redis is automatically added to the server if a value is provided for the `REDIS_URL` environment variable.
To invoke this data in a handler, simply add `cache: Cache` to the handler's signature.

### Helper Functions

#### get(cache: Cache, key: &str) -> Result<String, ApiError>

Retrieves a copy of the entry in the application cache by key.

Example:

```rust
use crate::cache::{get, Cache};

pub async fn handle(cache: Cache) -> impl Responder {
  let key = "SOME_KEY";
  let value = get(cache, key).await?;
  assert_eq!(value, "123");
}
```

#### set(cache: Cache, key: &str, value: &str) -> Result<String, ApiError>

Inserts or updates an entry in the application cache.

Example:

```rust
use crate::cache::{set, Cache};

pub async fn handle(cache: Cache) -> impl Responder {
  let key = "SOME_KEY";
  set(cache, key, "123").await?;
}
```

#### delete(cache: Cache, key: &str) -> Result<String, ApiError>

Deletes an entry in the application cache by key.

Example:

```rust
use crate::cache::{delete, Cache};

pub async fn handle(cache: Cache) -> impl Responder {
  let key = "SOME_KEY";
  delete(cache, key).await?;
}
```

## Non-Blocking Diesel Database Operations

When accessing a database via Diesel, operations block the main server thread.
This blocking can be mitigated by running the blocking code in a thread pool from within the handler.

Example:

```rust
pub async fn get_user(
    user_id: Path<Uuid>,
    pool: Data<PoolType>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = block(move || find(&pool, *user_id)).await?;
    respond_json(user)
}
```

Blocking errors are automatically converted into ApiErrors to keep the api simple:

```rust
impl From<BlockingError<ApiError>> for ApiError {
    fn from(error: BlockingError<ApiError>) -> ApiError {
        match error {
            BlockingError::Error(api_error) => api_error,
            BlockingError::Canceled => ApiError::BlockingError("Thread blocking error".into()),
        }
    }
}
```

## Endpoints

### Healthcheck

Determine if the system is healthy.

`GET /health`

#### Response

```json
{
  "status": "ok",
  "version": "0.1.0"
}
```

Example:

```shell
curl -X GET http://127.0.0.1:3000/health
```

### Login

`POST /api/v1/auth/login`

#### Request

| Param    | Type   | Description              | Required | Validations           |
| -------- | ------ | ------------------------ | :------: | --------------------- |
| email    | String | The user's email address |   yes    | valid email address   |
| password | String | The user's password      |   yes    | at least 6 characters |

```json
{
  "email": "torvalds@transmeta.com",
  "password": "123456"
}
```

#### Response

Header

```json
HTTP/1.1 200 OK
content-length: 118
content-type: application/json
set-cookie: auth=COOKIE_VALUE_HERE; HttpOnly; Path=/; Max-Age=1200
date: Tue, 15 Oct 2019 02:04:54 GMT
```

Json Body

```json
{
  "id": "0c419802-d1ef-47d6-b8fa-c886a23d61a7",
  "first_name": "Linus",
  "last_name": "Torvalds",
  "email": "torvalds@transmeta.com"
}
```

**When sending subsequent requests, create a header variable `cookie` with the value `auth=COOKIE_VALUE_HERE`**

### Logout

`GET /api/v1/auth/logout`

#### Response

`200 OK`

Example:

```shell
curl -X GET http://127.0.0.1:3000/api/v1/auth/logout
```

### Get All Users

Retrieve a paginated listing of all users in the system.

`GET /api/v1/user`

#### Query Parameters

| Param    | Type | Description                                     |
| -------- | ---- | ----------------------------------------------- |
| page     | i64  | The page to start on. Defaults to 1.            |
| per_page | i64  | The number of results per page. Defaults to 10. |

#### Response

```json
{
  "links": {
    "base": "http://127.0.0.1:3000/api/v1/user",
    "first": "http://127.0.0.1:3000/api/v1/user?page=1&per_page=10",
    "last": "http://127.0.0.1:3000/api/v1/user?page=13&per_page=10",
    "prev": null,
    "next": "http://127.0.0.1:3000/api/v1/user?page=2&per_page=10"
  },
  "pagination": {
    "offset": 0,
    "page": 1,
    "per_page": 10,
    "total": 129,
    "total_pages": 13
  },
  "data": [
    {
      "id": "00000000-0000-0000-0000-000000000000",
      "first_name": "admin",
      "last_name": "user",
      "email": "admin@admin.com"
    },
    {
      "id": "035efb82-cfdf-42de-adef-c75d7ac6d3ff",
      "first_name": "ModelUpdateaaa",
      "last_name": "TestUpdatezzz",
      "email": "model-update-test@nothing.org"
    }
  ]
}
```

Example:

```shell
curl -X GET http://127.0.0.1:3000/api/v1/user
```

### Get a User

`GET /api/v1/user/{id}`

#### Request

| Param | Type | Description   |
| ----- | ---- | ------------- |
| id    | Uuid | The user's id |

#### Response

```json
{
  "id": "a421a56e-8652-4da6-90ee-59dfebb9d1b4",
  "first_name": "Satoshi",
  "last_name": "Nakamoto",
  "email": "satoshi@nakamotoinstitute.org"
}
```

Example:

```shell
curl -X GET http://127.0.0.1:3000/api/v1/user/a421a56e-8652-4da6-90ee-59dfebb9d1b4
```

#### Response - Not Found

`404 Not Found`

```json
{
  "errors": ["User c63d285b-7794-4419-bfb7-86d7bb3ff17a not found"]
}
```

### Create a User

`POST /api/v1/user`

#### Request

| Param      | Type   | Description              | Required | Validations           |
| ---------- | ------ | ------------------------ | :------: | --------------------- |
| first_name | String | The user's first name    |   yes    | at least 3 characters |
| last_name  | String | The user's last name     |   yes    | at least 3 characters |
| email      | String | The user's email address |   yes    | valid email address   |

```json
{
  "first_name": "Linus",
  "last_name": "Torvalds",
  "email": "torvalds@transmeta.com"
}
```

#### Response

```json
{
  "id": "0c419802-d1ef-47d6-b8fa-c886a23d61a7",
  "first_name": "Linus",
  "last_name": "Torvalds",
  "email": "torvalds@transmeta.com"
}
```

Example:

```shell
curl -X POST \
  http://127.0.0.1:3000/api/v1/user \
  -H 'Content-Type: application/json' \
  -d '{
    "first_name": "Linus",
    "last_name": "Torvalds",
    "email": "torvalds@transmeta.com"
}'
```

#### Response - Validation Errors

`422 Unprocessable Entity`

```json
{
  "errors": [
    "first_name is required and must be at least 3 characters",
    "last_name is required and must be at least 3 characters",
    "email must be a valid email"
  ]
}
```

### Update a User

`PUT /api/v1/{id}`

#### Request

Path

| Param | Type | Description   |
| ----- | ---- | ------------- |
| id    | Uuid | The user's id |

Body

| Param      | Type   | Description              | Required | Validations           |
| ---------- | ------ | ------------------------ | :------: | --------------------- |
| first_name | String | The user's first name    |   yes    | at least 3 characters |
| last_name  | String | The user's last name     |   yes    | at least 3 characters |
| email      | String | The user's email address |   yes    | valid email address   |

```json
{
  "first_name": "Linus",
  "last_name": "Torvalds",
  "email": "torvalds@transmeta.com"
}
```

#### Response

```json
{
  "id": "0c419802-d1ef-47d6-b8fa-c886a23d61a7",
  "first_name": "Linus",
  "last_name": "Torvalds",
  "email": "torvalds@transmeta.com"
}
```

Example:

```shell
curl -X PUT \
  http://127.0.0.1:3000/api/v1/user/0c419802-d1ef-47d6-b8fa-c886a23d61a7 \
  -H 'Content-Type: application/json' \
  -d '{
    "first_name": "Linus",
    "last_name": "Torvalds",
    "email": "torvalds@transmeta.com"
}'
```

#### Response - Validation Errors

`422 Unprocessable Entity`

```json
{
  "errors": [
    "first_name is required and must be at least 3 characters",
    "last_name is required and must be at least 3 characters",
    "email must be a valid email"
  ]
}
```

#### Response - Not Found

`404 Not Found`

```json
{
  "errors": ["User 0c419802-d1ef-47d6-b8fa-c886a23d61a7 not found"]
}
```

### Delete a User

`DELETE /api/v1/user/{id}`

#### Request

| Param | Type | Description   |
| ----- | ---- | ------------- |
| id    | Uuid | The user's id |

#### Response

```json
{
  "id": "a421a56e-8652-4da6-90ee-59dfebb9d1b4",
  "first_name": "Satoshi",
  "last_name": "Nakamoto",
  "email": "satoshi@nakamotoinstitute.org"
}
```

#### Response

`200 OK`

Example:

```shell
curl -X DELETE http://127.0.0.1:3000/api/v1/user/a421a56e-8652-4da6-90ee-59dfebb9d1b4
```

#### Response - Not Found

`404 Not Found`

```json
{
  "errors": ["User c63d285b-7794-4419-bfb7-86d7bb3ff17a not found"]
}
```

## License

This project is licensed under:

- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
