# migren
**Migren** is a small migration tool for relational databases.

# usage
Firstly, move to directory, where you want to store your migrations *(or use `-d` argument and pass path to this directory)*

## .env variables
You need to export this environment variables:
```sh
DATABASE_URL="postgres://..." # You can use many drivers. Driver is recognised by schema
RUST_LOG=info # If it is not info, migren will not show you anything except the errors.
```

You can create `.env` file and migren will load this variables from it.

## new
Command `new` is creating new migration files.
```sh
# first_migration - name of migration
migren -d migrations new first_migration
```

It will create files `1_first_migration_up.sql` and `1_first_migration_down.sql` in `migrations` directory.
`1` - is an migration ID.
In `*_up.sql` file you define your changes.
In `*_down.sql` you define your rollback queries.

## top
To update database to the last migration, you can use `top` command:
```sh
migren top
```

`top` command uses transactions to update DB, so if anything went wrong, you will stay at the last migration.

## to
With `to` command you can move to neede migration (you can use it as rollback too)
```sh
# Move to migration with id 3
migren to 3
```

`to` command uses transactions to update DB, so if anything went wrong, you will stay at the last migration.

## status
Status command can show status of your migrations and database:
```sh
migren status
```
