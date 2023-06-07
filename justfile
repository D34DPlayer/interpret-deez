default_tag := 'crh/interpret-deez'

default:
  @just --list --justfile {{justfile()}}

# Build native
build:
    cargo build --release

# Run native
run:
    cargo run

# Build container
build-container tag=default_tag:
    docker build -t {{tag}} .

# Run container
run-container tag=default_tag:
    docker run -ti {{tag}}