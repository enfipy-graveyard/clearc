# Rust Clean Architecture

Project `clearc` - is an example of implementation of Clean Architecture in Rust language

## Usage:

To begin development just run:

```bash
docker-compose up --build
```

## Todo:

1. Write more detailed README
2. Add database migrations
3. Finish actix setup

## Production build:

To build lightweight production image under just run:

```
docker build -f docker/prod.Dockerfile -t test --build-arg PROJECT=clearc .
```
