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


## API Endpoints

The application runs on port 5000 by default.

The root url for all endpoints is http://localhost:5000


- POST `/user`
```
Body : { username | String , password | String }
```
This endpoint is for registering a new user.


- POST `/login`
```
Body : { username | String , password | String }
```
This endpoint returns a jsonwebtoken on successful login.
Imp Note ! This token must be present in Auth header of all subsequent requests.


- GET `/all`
```
Headers : { Authorization : Bearer <token> }
```
Returns all task items for currently logged-in user.


- POST `/create`
```
Body : { name | String , description | String , deadline | String }
Headers : { Content-Type : application/json , Authorization : Bearer <token> }
```
Create a new task. All feilds are required.


- PUT `/update/<id>`
```
Body : { name | String , description | String , deadline | String }
Headers : { Content-Type : application/json , Authorization : Bearer <token> }
```
Partially update an existing task for the current user based on id (passed in url). All feilds (in body) are optional and any combination of them may be present.


- DELETE `/delete/<id>`
```
Headers : { Authorization : Bearer <token> }
```
Delete any task of the current user based on id (passed in url)