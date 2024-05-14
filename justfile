dev:
    trunk serve

check:
    cargo check --target=wasm32-unknown-unknown

deploy:
    trunk build --release
    wrangler pages deploy --project-name=lint ./dist
