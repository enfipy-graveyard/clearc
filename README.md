# Rust Clean Architecture

Project `clearc` - is an example of implementation of "Clean Architecture" by Uncle Bob in Rust language. Feel free to contribute.

## Usage:

To begin development just run:

```bash
docker-compose up --build
```

## Todo:

1. Write more detailed README
2. Finish To-do base logic

## Production build:

To build lightweight production image run:

```
docker build -f docker/prod.Dockerfile -t test --build-arg PROJECT=clearc .
```
