# XChemLab

This repository contains a collection of services which comprise the XChemLab deployment.

## Running

To run any of the services, simply execute `cargo run -r <SERVICE_NAME>`.

Note, deployments of the following ancillary services are required:
- An [Open Policy Agent](https://www.openpolicyagent.org/) instance serving the polcies in `/policies` is for authentication & authorization.
- A [PostgreSQL](https://www.postgresql.org/) instance to provide a database backend for various services.
- A [RabbitMQ](https://www.rabbitmq.com/) instance to communicate between services and analysis workers (e.g. `chimp_chomp`).

## Developing

A [devcontainer](https://containers.dev/) docker-compose configuration is provided in `.devcontainer` to aid development. If developing in VSCode this can be activated with the Dev Containers extension (`ms-vscode-remote.remote-containers`).

## Test Data

A helper script, `collect_test_data.sh`, is included. This can be used to collect copies of all recent SoakDB databases into the `test_data` directory.
