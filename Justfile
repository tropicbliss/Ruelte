_default:
  just --list

setup:
  cargo install cargo-watch systemfd just typeshare-cli
  cd client && npm i

watch:
    cd client && npm run watch