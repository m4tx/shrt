shrt
====

[![Rust Build Status](https://github.com/m4tx/shrt/workflows/Rust%20CI/badge.svg)](https://github.com/m4tx/shrt/actions/workflows/rust.yml)
[![Docker Build Status](https://github.com/m4tx/shrt/workflows/Docker/badge.svg)](https://github.com/m4tx/shrt/actions/workflows/docker-publish.yml)
[![GNU AGPL v3 licensed](https://img.shields.io/github/license/m4tx/shrt)](https://github.com/m4tx/shrt/blob/master/LICENSE)

Shrt is a modern link shortener service written in Rust. It is designed to be fast, secure, and easy to use. It is composed of two parts: the backend, which is a RESTful API, and the frontend, which is a single-page application.

## Development

The project is written purely in [Rust](https://www.rust-lang.org/), both its backend and frontend.

### Backend

To run the development server, execute:

```shell
cd shrt-backend
cargo run
```

This will start the server at [localhost:8000](http://localhost:8000).

To build a release version, execute:

```shell
cargo build --release
```

The target binary will be put at `target/release/shrt-backend`.

### Frontend

Frontend uses the [yew](https://yew.rs/docs/getting-started/build-a-sample-app) framework. The code is compiled into a WebAssembly binary and then statically served.

First, install the [Trunk](https://trunkrs.dev/) bundler and add wasm32 target support to your Rust toolchain.

```shell
rustup target add wasm32-unknown-unknown
cargo install trunk
```

Then, you can use:

```shell
cd shrt-frontend
trunk serve
```

to start local server at [localhost:8080](http://localhost:8080). It assumes that the backend is running at [localhost:8000](http://localhost:8000).

To build a distributable version of the frontend, execute:

```shell
trunk build --release
```

This will build a website in `frontend/dist/` directory that can be statically served by a server such as nginx. You can override th backend URL by providing it as the `SHRT_API_URL` environment variable, like so:

```sh
export SHRT_API_URL=http://api.shrt.example.com
trunk build --release
```

#### `pre-commit`
We encourage contributors to use predefined [`pre-commit`](https://pre-commit.com/) hooks — to install them in your local repo, make sure you have `pre-commit` installed and run:

```shell
pre-commit install
```

## Deployment

The easiest way to try locally or deploy _shrt_ is to use auto-generated Docker images. There is a separate image for backend, frontend, and a reverse proxy (that exposes both the backend and frontend under the same server), all of which are published on [the GitHub Container Registry](https://github.com/m4tx?tab=packages&repo_name=shrt). There is an example `docker-compose.yml` file provided in the repository root.

In the project root directory, execute:

```shell
docker compose up -d
```

After that, the website will be available on at [localhost:8000](http://localhost:8000).
