# Vehicle Listing API

Basic API for listing vehicles with Actix framework. All data are stored on MongoDB.

# Usage

### Connecting to MongoDB

You can use a local server for MongoDB, or you can you get a URL on [Atlas](https://www.mongodb.com/atlas).

---

### Set the env variables

The environment variables should be stored in the `/.cargo/config.toml` file as in the [documentation](https://doc.rust-lang.org/cargo/reference/config.html#configuration).

Yor `config.toml` file should look something like this at the end:

``` toml
[env]
DB_URL="URL to your Mongo db"
```
---

### Run the API

To execute the example code, run `cargo run` in the root directory.
