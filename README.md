# Zed Postgres Context Server

This extension provides a Model Context Server for Postgres, for use with the Zed AI assistant.

It adds a `/pg-schema` slash command to the Assistant Panel.

## Configuration

To use the extension, you will need to point the context server at a Postgres database by setting the `database_url` in your Zed `settings.json`:

```json
{
  "context_servers": {
    "postgres-context-server": {
      "settings": {
        "database_url": "postgresql://myuser:mypassword@localhost:5432/mydatabase"
      }
    }
  }
}
```

## Usage

- `/pg-schema <table-name>`: Retrieve the schema for the table with the given name.
- `/pg-schema all-tables`: Retrieve the schemas for all tables in the database.
