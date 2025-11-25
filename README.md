## CLI Database Engine
#### What Is a CLI Database Engine?

A CLI Database Engine is a small, command-line program that works like a simplified version of databases such as Redis or SQLite, but much smaller and easier to understand.

You run the program in your terminal, and you can store, retrieve, update, and delete data using text commands like:

set username james
get username
delete username
keys
save


#### The system acts like a mini database that:

✔ Stores data in memory
✔ Lets users run commands to interact with the data
✔ Saves the data to a file so it persists
✔ Loads the data again when restarted

This project helps a developer understand how databases work internally while practicing Rust concepts like ownership, memory safety, error handling, and concurrency.

## Project Goals

- The goal is to build a lightweight, key-value database from scratch.

### The engine should:

- Accept commands from a user via the command line

- Keep the data in-memory using Rust structures (like HashMap)

- Save the data to a file in JSON or another format

- Load the data back when the program restarts

- Handle errors cleanly and safely

- Provide a consistent interface for interacting with the database

- The entire program runs inside the terminal — there is no web UI.

## Core Features
1. Set a Value

Store a pair of values (like a variable):
```
set name Victor
```

2. Get a Value

Retrieve a stored value:
```
get name

```
3. Delete a Key

Remove a key and its value:
```
delete name
```
4. List All Keys

Show all stored keys:
```
keys
```
5. Save to Disk

Write the current database content to a file:

save

6. Load from Disk

Load previously saved data when the program starts.