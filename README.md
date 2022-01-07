# pandoc-rustful-api

[![Rust](https://github.com/friessec/pandoc-rustful-api/actions/workflows/rust.yml/badge.svg)](https://github.com/friessec/pandoc-rustful-api/actions/workflows/rust.yml)

This project is a pandoc RESTful API written in Rust.
It is intended as a background service to generate documents from provided Markdown files.

Currently, the project is in an early stage of development.
API and data processing will change frequently.

## Building and Testing

```shell
docker build -t pandoc-rustful-api .
docker run -p 8000:8000 pandoc-rustful-api
```

### Swagger-UI

For development purposes, Swagger-UI can be used to get an api overview.
The UI is exposed at http://localhost:8080/swagger-ui/.
