# A Rust service template

A template for a Rust service that comes with:

1. Postgres integration with sqlx - with some sane defaults & efficient patterns
2. Server implementation with poem & poem-openapi (swagger explorer included)
3. Observability integration with
  3.1. Distributed tracing support for Datadog, Jaeger & OpenTelemetry via fastrace
  3.2. TODO: Metrics?!?
  3.3. Logging via logforth (JSON logging not yet implemented)
4. Config management via `config`
5. Easy dev environment setup

## Getting started

Clone the template with:

```bash
git clone --depth 1 git@github.com:Dzejkop/rust-service-template.git; rm -rf ./rust-service-template/.git
```

Make sure you have mise & docker with docker-compose installed and then run `just dev-up`. This command will provision the database including migrations. Next you can simply run:

```bash
just dev
```

This command will install the required tools (sqlx & cargo-watch), setup the db via docker-compose and start the server in watch mode.

## Observability

What this template does not include is a dev observability stack. Instead I recommend the [observability-toolkit](https://github.com/cbos/observability-toolkit).

The `.env.dev` file is set up to send traces to the local OpenTelemetry collector instance.
