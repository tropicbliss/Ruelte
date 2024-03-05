# Svelte + Rust

# Setup instructions

## Linux / macOS (Bash compatible shell)
```sh
cargo install cargo-watch systemfd
cd client && npm i
cd ../
./watch.sh
```

## Windows (PowerShell)
```sh
cargo install cargo-watch systemfd
cd client ; npm i
cd ../
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
./watch.ps1
```
