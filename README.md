
<p align="center">
    <img src="https://user-images.githubusercontent.com/7833164/86766972-44d8dd80-c019-11ea-91a6-e893095a41a7.png" height="200" weight="200"></img>
</p>

[![Build Status](https://travis-ci.com/Project-HQ/HQ-Core.svg?branch=master)](https://travis-ci.com/Project-HQ/HQ-Core)
# HQ-Core 

  A Rusty RESTful Postgres Microservice 
  
**THIS IS A WORK IN PROGRESS**

If you've come across this repo, feel free to reach out and ask about the project. However, it is not currently ready for use. This message will be removed once we are ready for user testing and review. 

## How to Run

1. Create a postgresql server with an empty database on a linux box, name the table hqcore and create a user that is allowed to have remote access to the table.
2. Add your connection info to the env folder of HQ-Core:
  `cd HQ-CORE && echo DATABASE_URL=postgres://postgres_username:postgres_passwd@hostname:5432/hqcore > .env`
3. Get your dependencies: `cargo build`
4. Setup the db with diesel: `diesel setup`
5. Run any migrations (if changes were made): `diesel migration run`
6. Server should now be available on port 8080 over http

## Dockerize It

You will want to use rust-musl-builder to create a lightweight image with no dependencies. Once the image is built, run it:
`docker run --env-file=./path/to/.env --name hq-core -p 8080:8080 hq-core`

[Tutorial Here](https://blog.semicolonsoftware.de/building-minimal-docker-containers-for-rust-applications/)

## Documentation

Check our [Postman](https://documenter.getpostman.com/view/11957802/T17J9SWE) for the API Docs.
