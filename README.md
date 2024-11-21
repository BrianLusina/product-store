# Product Store

## Setup & Requirements

Ensure you have the following setup on your local development environment to ensure that the project runs smoothly on
your system.

### [Rust](https://www.rust-lang.org)

This is the programming language used to build the project. Ensure you have it locally installed on your system following
the instructions in the link provided.

### [Diesel CLI](https://diesel.rs/guides/getting-started)

[Diesel](https://diesel.rs) is a safe, extensible ORM and query builder for Rust and will require the CLI installed locally
for use. Follow the instructions defined in the link provided to get this setup.

### [Docker](https://docs.docker.com/desktop/)

Docker is required to run the database and other services locally as specified in the [docker-compose file](./docker-compose.yml).
Follow along with the instructions in the link to install docker locally. If you prefer to run the services natively(i.e. not using Docker),
then follow along with how to install those services natively on your local machine.

## Running The Project

To get the project running after initializing all the required dependencies and tools, run the following commands.

### Startup the database plus other services

If using the [Docker](#docker) setup option, run the below command to spin up the services:

```shell
docker compose up
```

> This will start the services required in the current shell

If you want the services to run in the background:

```shell
docker compose up -d
```

> This starts up the docker containers in detached mode(i.e. in the background)

### Setup the database with Diesel CLI

Once the [Diesel CLI](#diesel-cli) has been setup and complete, run the below command:

```shell
make diesel-setup
```

> This will set up the database using diesel cli

#### Running migrations

In order to run migrations, there is a handy command in the [Makefile](./Makefile) that will make it easy to create migration
scripts using Diesel:

```shell
make create-migration migration-name=<NAME_OF_MIGRATION>
```

> <NAME_OF_MIGRATION> is the directory name or the name of the migration to be created. Note that the `migration-name`
> is an argument that is required.

This will create a new migration folder in the [migrations](./migrations) folder:

```plain
migrations
├── 00000000000000_diesel_initial_setup
│    ├── down.sql
│    └── up.sql
└── 2024-11-21-061319_create_products
    ├── down.sql
    └── up.sql
```

> This is an example when run with `make create-migration migration-name=create_products`

From there, open up the `up.sql` & `down.sql` files and write up the SQL script to create tables, functions, triggers, etc.

Note that the `down.sql` undoes what has been setup in the `up.sql`. 

Next up is to run the migrations. This can be done with the command:

```shell
make run-migrations
```

> This will run all the migrations not already run

To undo a migration:

```shell
make redo-migrations
```

> This will undo a migration

## Tools used

- [Rust](https://www.rust-lang.org) - Programming Language
- [Diesel](https://diesel.rs) - Declarative ORM
- [Actix Web](https://actix.rs/) - Web Framework
- 