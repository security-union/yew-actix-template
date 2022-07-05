# Yew  + Actix Full Stack Template
<p align="center">
  <img src="https://user-images.githubusercontent.com/1176339/177201719-cd387dae-fdd0-4237-90ec-f140fcfcb49c.png" width="400"/>
</p>

## YouTube video
https://www.youtube.com/watch?v=oCiGjrpGk4A


Contains 3 sub-projects

1. actix-api: actix web server
2. yew-ui: Yew frontend
3. types: json serializable structures used to communicate the frontend and backend.

Execute `./start_dev.sh` to start all components.

Do a code change to to the yew-ui, types or actix-api and see how everything reloads.

# Prerequisites

1. Install rust, cargo and friends. Please watch this video for more details: https://youtu.be/nnuaiW1OhjA
https://doc.rust-lang.org/cargo/getting-started/installation.html

2. Install trunk and `target add wasm32-unknown-unknown` please watch this video for more details: https://youtu.be/In09Lgqxp6Y
```
cargo install --locked trunk
target add wasm32-unknown-unknown
```

3. Install cargo watch 
```
cargo install cargo-watch
```
