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

> This will setup the database using diesel cli