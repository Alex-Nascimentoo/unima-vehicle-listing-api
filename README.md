# Vehicle Listing API

Basic API for listing vehicles with Actix framework. All data are stored on MongoDB.

# Usage

### Connecting to MongoDB

You can use a local server for MongoDB, or you can you get a URL on [Atlas](https://www.mongodb.com/atlas).

##

### Set the env variables

The environment variables should be stored in the `/.cargo/config.toml` file as in the [documentation](https://doc.rust-lang.org/cargo/reference/config.html#configuration).

Yor `config.toml` file should look something like this at the end:

``` toml
[env]
DB_URL="URL to your Mongo db"
```
##

### Run the API

To execute the example code, run `cargo run` in the root directory.

### Routes and responses

__GET METHOD__ `http://localhost:7001/api/vehicle` will return a JSON list of type vehicle:

```javascript
{
  "_id": {
    "$oid": "6532cc986db86fea8ee842c5"
  },
  "brand": "Mercedes",
  "model": "GLA 200",
  "color": "silver",
  "price": "505",
  "wheels": 4,
  "doors": 4,
  "is_available": true
}
```

##

__GET METHOD__ `http://localhost:7001/api/vehicle/{id}` will return a JSON with single vehicle.

##

__POST METHOD__ `http://localhost:7001/api/vehicle` requires a JSON body of type vehicle:

```javascript
{
  "v_type": "{vehicle}", // car | motorcycle
  "brand": "{vehicle brand}", // Porsche
  "model": "{vehicle model}", // Cayenne
  "color": "{vehicle color}", // Midnight blue
  "price": "{vehicle price}" // Accept string format
}
```
This route will return a json with simple string confirmation

##

__PUT METHOD__ `http://localhost:7001/api/vehicle/update` requires a JSON body of custom type:

```javascript
{
  "id": "{vehicle id}",
  "brand": "{vehicle brand}",
  "model": "{vehicle model}",
` "color": "{vehicle color}",
  "price": "{vehicle price}" // This field should be of type string
}
```
This route will return a json with simple string confirmation




