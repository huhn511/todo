# Rust ToDo App

you need to create an `.env`variable with this content:
```bash
SERVER.HOST=0.0.0.0
SERVER.PORT=3000
```

start the application
```bash
cargo run
```

### Docker

build and run in docker
```bash
docker build -t=todo .
docker run -t todo:latest
```