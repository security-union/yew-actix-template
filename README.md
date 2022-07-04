# Actix  + Yew Full Stack Template

Contains 3 sub-projects

1. actix-api: actix web server
2. yew-ui: Yew frontend
3. types: json serializable structures used to communicate the frontend and backend.

Execute `./start_dev.sh` to start all components.

Do a code change to to the yew-ui, types or actix-api and see how everything reloads.

# Prerequisites

1. Install trunk and `target add wasm32-unknown-unknown` please watch this video for more details: https://youtu.be/In09Lgqxp6Y
```
cargo install --locked trunk
target add wasm32-unknown-unknown
```

2. Install cargo watch 
```
cargo install cargo-watch
```
