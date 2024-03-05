# Svelte + Rust

## Setup instructions

### Linux / macOS (Bash compatible shell)
```sh
cargo install cargo-watch systemfd
cd client && npm i
cd ../
./watch.sh
```

### Windows (PowerShell)
```sh
cargo install cargo-watch systemfd
cd client ; npm i
cd ../
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
./watch.ps1
```

### Deploy project to a Docker container

```sh
cargo install cargo-chef
cargo chef cook --release --recipe-path recipe.json
docker build -t your-image-name:tag .
docker run -p 3000:3000 your-image-name
```
