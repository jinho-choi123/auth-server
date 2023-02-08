# auth-server

## Getting started with Docker(w/o Docker compose)
use Docker to easily setup authentication server.

1. Docker pull image.
```
$ docker pull choijinho817/auth-server:latest
```
2. Run docker image with following environment variables set.
```
$ docker run \
-e MONGO_URL=##MONGO_URI## \
-e DB_PWD_SALT1=##RANDOMSALT## \
-e DB_PWD_SALT2=##RANDOMSALT2## \
-e JWT_SALT=##RANDOMJWTSALT## \
-e ACCESS_JWT_LIFETIME=##ACCESS_JWT_LIFETIME(seconds)## \
-e REFRESH_JWT_LIFETIME=##REFRESH_JWT_LIFETIME(seconds)## \
-e PORT=##PORT## \
choijinho817/auth-server:latest
```

## Getting started with Docker Compose(Recommended)
1. Docker pull image.
```
$ docker pull choijinho817/auth-server:latest
```
2. Create .env file. It should be like the following.
```
# env variables for database setup
MONGO_ROOT_USERNAME='@@random username@@'
MONGO_ROOT_PASSWORD='@@random hash@@'

# env variables for auth-server setup
MONGO_URL='@@mongo db uri@@'
DB_PWD_SALT1='@@password hash1@@'
DB_PWD_SALT2='@@password hash2@@'
JWT_SALT='@@jwt salt@@'
ACCESS_JWT_LIFETIME='@@access token lifetime in seconds@@'
REFRESH_JWT_LIFETIME='@@refresh token lifetime in seconds@@'
PORT='@@service port@@'
```
3. Create docker-compose.yml file in the same directory of .env file. The content should look like the following:
```
version: '3.8'

services:
  auth_server: 
    image: choijinho817/auth-server:1.3 ##local custom image https://gitlab.com/-/snippets/2476117
    container_name: authServer
    ports: 
      - 9090:9090
    environment:
      - MONGO_URL=${MONGO_URL}
      - DB_PWD_SALT1=${DB_PWD_SALT1}  
      - DB_PWD_SALT2=${DB_PWD_SALT2}
      - JWT_SALT=${JWT_SALT}
      - ACCESS_JWT_LIFETIME=${ACCESS_JWT_LIFETIME}
      - REFRESH_JWT_LIFETIME=${REFRESH_JWT_LIFETIME}
      - PORT=${PORT}
    networks: 
      - authNet 
  mongo:
    image: mongo 
    restart: always
    container_name: mongo
    environment:
      MONGO_INITDB_ROOT_USERNAME: ${MONGO_ROOT_USERNAME}
      MONGO_INITDB_ROOT_PASSWORD: ${MONGO_ROOT_PASSWORD}
    networks: 
      - authNet 
  mongo-express:
    image: mongo-express 
    restart: always 
    ports: 
      - 8081:8081
    environment:
      - ME_CONFIG_MONGODB_ADMINUSERNAME=${MONGO_ROOT_USERNAME}
      - ME_CONFIG_MONGODB_ADMINPASSWORD=${MONGO_ROOT_PASSWORD}
      - ME_CONFIG_MONGODB_URL=${MONGO_URL}
    networks: 
      - authNet


networks:
  authNet:
    driver: bridge
```

## Details about Environment Variables

|NAME|Description|Default Value|
|----|-----------|-------------|
|MONGO_URL|MONGO DB URI for storing user infos(https://www.mongodb.com/docs/manual/reference/connection-string/)|None|
|DB_PWD_SALT1|Front padding to user password hash salt. Set to random salt as default|Randomly set as default|
|DB_PWD_SALT2|Back padding to user password hash salt. Set to random salt as default|Randomly set as default|
|JWT_SALT|JWT Hash Salt. Set to random salt as default|Randomly set as default|
|ACCESS_JWT_LIFETIME|Access JWT's lifetime in seconds. Usually 60 minutes. Set to 60 minutes as default.|3600|
|REFRESH_JWT_LIFETIME|Refresh JWT's lifetime in seconds. Usually 24 hours. Set to 86400 as default.|
|PORT|Port of container that authentication service is running. Set to 9090 as default|9090|
|MONGO_ROOT_USERNAME|See https://hub.docker.com/_/mongo for more info|None|
|MONGO_ROOT_PASSWORD|See https://hub.docker.com/_/mongo for more info|None|

## API Docs
#### check server status
<details>
 <summary><code>GET</code> <code><b>/checkserver</b></code> <code>(get server status and server time)</code></summary>

 #### Request Body
> |name|type|data type|description|
> |----|----|---------|-----------|
> |None|None|None|None|
#### Parameters
> |name|type|data type|description|
> |----|----|---------|-----------|
> |None|None|None|None|
#### Response
>|http code|content type|response|
>|---------|------------|--------|
> |200|application/json|{"msg": "Server is running. Server time is 2023-02-07 11:49:14"}|
##### Example cURL
> ```javascript
>  curl --location --request GET 'http://localhost:9090/checkserver'
> ```
</details>

-----------------------------------------------------------------------------------

<details>
 <summary><code>POST</code> <code><b>/users/create</b></code> <code>(create user)</code></summary>

 #### Request Body(application/json)
> |name|type|description|
> |----|----|-----------|
> |email|string|User email|
> |password|string|User password|
#### Parameters
> |name|type|data type|description|
> |----|----|---------|-----------|
> |None|None|None|None|
#### Response
>|http code|content type|response|
>|---------|------------|--------|
> |200|application/json|{"msg":"create user success","email":"test@test.test"}|
> |500|application/json|{"error":"Duplicate email found. Please check your email"}|
##### Example cURL
> ```javascript
>  curl --location --request POST 'http://localhost:9090/users/create' \
>   --header 'Content-Type: application/json' \
>   --data-raw '{
>      "email": "test@test.test",
>       "password": "testpassword"
>     }'
> ```
</details>

-----------------------


<details>
 <summary><code>DELETE</code> <code><b>/users/delete/:userEmail</b></code> <code>(delete user)</code></summary>

 #### Request Body(application/json)
> |name|type|description|
> |----|----|-----------|
> |None|None|None|
#### Parameters
> |name|type|data type|description|
> |----|----|---------|-----------|
> |userEmail|string|email address|email address of user to delete|
#### Response
>|http code|content type|response|
>|---------|------------|--------|
> |200|application/json|{"msg":"User deletion success.","email":"test@test.test"}|
> |404|application/json|{"error":"User not found. Please check your email"}|
##### Example cURL
> ```javascript
>   curl --location --request DELETE 'http://localhost:9090/users/delete/test@test.test' 
> ```
</details>

-------------------

<details>
 <summary><code>POST</code> <code><b>/users/login</b></code> <code>(login user. get access token and refresh token)</code></summary>

 #### Request Body(application/json)
> |name|type|description|
> |----|----|-----------|
> |email|string|user's email|
> |password|string|user's password|
#### Parameters
> |name|type|data type|description|
> |----|----|---------|-----------|
> |None|None|None|None|
#### Response
>|http code|content type|response|
>|---------|------------|--------|
> |200|application/json|{"accessToken":"test.test.test","refreshToken":"test.test.test"}|
> |404|application/json|{"error":"User does not exist. Please check user email."}|
##### Example cURL
> ```javascript
>   curl --location --request POST 'http://localhost:9090/users/login' \
>     --header 'Content-Type: application/json' \
>     --data-raw '{
>       "email": "test@test.test",
>       "password": "testpassword"
>       }'
> ```
</details>

----------------

<details>
 <summary><code>GET</code> <code><b>/users/logout</b></code> <code>(logout user. Invalidate refresh token)</code></summary>

 #### Request Body(application/json)
> |name|type|description|
> |----|----|-----------|
> |email|string|user's email|
#### Parameters
> |name|type|data type|description|
> |----|----|---------|-----------|
> |None|None|None|None|
#### Response
>|http code|content type|response|
>|---------|------------|--------|
> |200|application/json|{"msg":"logout success.","email":"test@test.test"}|
> |404|application/json|{"error":"User does not exist. Failed storing refresh JWT to DB."}|
##### Example cURL
> ```javascript
>   curl --location --request GET 'http://localhost:9090/users/logout' \
>     --header 'Content-Type: application/json' \
>     --data-raw '{
>         "email": "jin@mail.dss"
>       }'
> ```
</details>

---------------------------

<details>
 <summary><code>GET</code> <code><b>/jwt/verify</b></code> <code>(verify access token contained in request headers - used as middleware in backend server)</code></summary>

#### Request Headers)
> |name|type|description|
> |----|----|-----------|
> |Authorization|string|request headers Authorization field contains access token. It starts with "Bearer". It has white space between "Bearer" and access jwt.|
#### Request Body(application/json)
> |name|type|description|
> |----|----|-----------|
> |None|None|None|
#### Parameters
> |name|type|data type|description|
> |----|----|---------|-----------|
> |None|None|None|None|
#### Response
>|http code|content type|response|
>|---------|------------|--------|
> |200|application/json|{"msg":"verification success. User is authorized.","email":"test@test.test"}|
> |500|application/json|{"error": "Error occur while decoding jwt."}|
##### Example cURL
> ```javascript
>   curl --location --request GET 'http://localhost:9090/jwt/verify' \
>     --header 'Authorization: Bearer test.test.test'
> ```
</details>

---------------------

<details>
 <summary><code>POST</code> <code><b>/jwt/refresh</b></code> <code>(convert refresh jwt to access jwt)</code></summary>

#### Request Body(application/json)
> |name|type|description|
> |----|----|-----------|
> |refresh_token|string|refresh jwt|
#### Parameters
> |name|type|data type|description|
> |----|----|---------|-----------|
> |None|None|None|None|
#### Response
>|http code|content type|response|
>|---------|------------|--------|
> |200|application/json|{"accessToken":"test.test.test","msg":"refreshing token success."}|
> |500|application/json|{"error":"Error occur while decoding jwt."}|
##### Example cURL
> ```javascript
>   curl --location --request POST 'http://localhost:9090/jwt/refresh' \
>     --header 'Content-Type: application/json' \
>     --data-raw '{
>         "refresh_token": "test.test.test"
>       }'
> ```
</details>