###Menu Service

Purpose of this service is to manage menus for the POS system

##Functionality
Currently supports three operations: 
- GET /menu/{id}
- POST /menu
- POST /item

##To Run Locally
- Run `docker build -t menu-service-app .` or run `./build.sh`
This builds the local image of the app (will take awhile)
- Run `docker-compose up -d` or `./run.sh`
- Tear down with `docker-compose down` or `./teardown.sh`
This runs the app and database in detached mode

##To develop
- You can run the local database for development by running `docker-compose -f docker/local.yml up -d` (there are shell scripts to run and teardown)
- You will need rustc installed (this will include cargo) I recommend using rustup (rustc version manager) for install.
- You will need to run `cargo install sqlx-cli --no-default-features --features=postgres` to get the sqlx cli
- You can run `sqlx database setup` to create the database and run the migrations against the running database
- The dev-db-run.sh will start the container and run the sqlx command
- You will need to set a .env file with the following variables:
    - DATABASE_URL=postgres://postgres:postgres@localhost:5432/menu
    - PORT=8080
    - HOST=127.0.0.1
## Sqlx does compile time checks against the code, this means you will not be able to compile the code unless the database is up and the schema is made
## If you make database changes, you must run `cargo sqlx prepare` to generate a json file of metadata so that the docker image build will not fail
