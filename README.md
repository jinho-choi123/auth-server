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