# rust-web-server

A rust based web server for implementing a Task List Application. For the REST APIs the **warp** framework is used. **Tokio** is used for asynchronous operations.
The REST API implementation has endpoints for: 
```
- registering a user
- logging in
- creating a task
- updating a task (partial)
- deleting a task.
```
For authentication **json web tokens** is used , ensuring that all task related endpoints are only accessible by logged in users (using a token).

An `HashMap` is used as an in memory database.


## Run Locally

A .env file needs to be present in the root of the project ( not in this repo ) with a line

```
JWT_SECRET="my_secret";
```

the string *"my_secret"* is used to encode the JWT data and decoding it during user authentication, it can be replaced by any string.

The program is run using the command: `cargo run` or `cargo run --release`
