# Rust GraphQL Server Demo

This is a basic database-backed GraphQL server implementation to demonstrate the
concepts.

## Prerequisites - database

You need to have a database running to run this app. Run the included script to
start a database in a docker container:

    $ ./db_create.sh

That creates a persistent PostgreSQL container with an exposed port,
credentials, and database name that match the values that the app expects as
configured in `.env`.

If you want the database to go away you will need to stop, and remove it:

    $ docker stop graphql_server_demo_db
    $ docker rm graphql_server_demo_db

Because the container is persistent, if it is stopped and not removed, or you
reboot or something you can get the database back with intact data by starting
it again.

    $ docker start graphql_server_demo_db

The app automatically creates the appropriate database schema when it is
started.

## Running

Run with:

    $ cargo run

Environment variables, such as the web app port and log levels, are read from
`.env`.

## Usage

You can open https://localhost:8000/ in a web browser to load the interactive
GraphQL playground. This lets you see the API schema, and run queries and
mutations, and so forth.

You can send GraphQL API requests to the same address.

## Database Migrations

If you want to make changes to the database schema the proper way to do that is
to write a migration. First make sure that you have `slqx-cli` installed:

    $ cargo install sqlx-cli

To create a migration run:

    $ sqlx migrate add -r <name for your migration>

Add some `create table` or `alter table` or what-have-you SQL statements to the
generated migration file. The app will automatically apply the migration the
next time you start it.

## Troubleshooting

If you see this error from the Rust compiler:

    error: error communicating with database: Connection refused (os error 111)

That means that you don't have the database running.
See the "Prerequisites - database" section above.

Sqlx provides some really nifty macros that query the database on-the-fly to
type-check database queries in Rust code. But that means that you need to have
a running database to build the program. It is possible to dump database
metadata into a JSON file that the macro will read instead of connecting to the
database. That would be convenient for someone checking out the project for the
first time - but it also adds a required step of updating that JSON file
whenever the database schema changes. See
https://github.com/launchbadge/sqlx/blob/master/sqlx-cli/README.md#enable-building-in-offline-mode-with-query
