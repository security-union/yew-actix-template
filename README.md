# Docker + Actix + Yew Full Stack Template
<p align="center">
  <img src="https://user-images.githubusercontent.com/1176339/179438927-e09e2f37-9144-41ba-a080-e2c3c250b113.png" width="400"/>
</p>

## 👨‍💻 YouTube videos
1. Full Stack Rust App Template using Yew + Actix! https://www.youtube.com/watch?v=oCiGjrpGk4A
2. Add Docker to your full stack Rust app Actix + Yew App https://youtu.be/YzjFk694bFM
3. SERVER SIDE OAUTH with Actix Web, Yew and Rust (analyzing GRAMMARLY) https://youtu.be/Wl8oj3KYqxM
4. I added a Database To Our YEW ACTIX Template To Store Users And OAuth Tokens.
 https://youtu.be/ENgMHIQk7T8
 
## 👉 Join our Discord Community
You can join our Discord Community, here is the [invite link]((https://discord.gg/JP38NRe4C).

## 👨‍💻 Project Structure

Contains 3 sub-projects

1. actix-api: actix web server
2. yew-ui: Yew frontend
3. types: json serializable structures used to communicate the frontend and backend.

# Dockerized workflow

1. Install docker

2. Run one of the supported make commands

```
make test
make up
make down
make build
```

# OAuth2

This template supports OAuth2 via yew-auth, to configure client_id and other secrets, read the docker-compose => 
https://github.com/security-union/yew-actix-template/blob/main/docker/docker-compose.yaml

Copy `docker/.env-sample` to `docker/.env` and fill in the variables.

# Native build

Execute `./start_dev.sh` to start all components.

Do a code change to to the yew-ui, types or actix-api and see how everything reloads.

# Prerequisites (native)

1. Install rust, cargo and friends. Please watch this video for more details: https://youtu.be/nnuaiW1OhjA
https://doc.rust-lang.org/cargo/getting-started/installation.html

2. Install trunk and `target add wasm32-unknown-unknown` please watch this video for more details: https://youtu.be/In09Lgqxp6Y
```
cargo install --locked trunk
rustup target add wasm32-unknown-unknown
```

3. Install cargo watch 
```
cargo install cargo-watch
```
