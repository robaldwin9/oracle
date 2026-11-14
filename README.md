# Oracle

A Rust Discord bot using Poise and slash commands.

## Configuration

Copy `.env.example` to `.env` and set `DISCORD_TOKEN`. Never commit the real
token. Leave privileged Discord gateway intents disabled for the current
slash-command-only bot.

Each file in `data/` contains one response per line:

- `compliments.txt`
- `insults.txt`
- `answers.txt`

The bot loads these files at startup. Blank lines are skipped; empty lists
are rejected.

## Run locally

From the project root:

```bash
cargo run
```

`DATA_DIR` defaults to `./data`. Relative paths resolve from the working
directory.

## Docker

With Docker Engine and the Compose plugin installed:

```bash
docker compose up --build -d
docker compose logs -f oracle
```

Compose reads the token from the host environment or `.env` and passes it to
the container. The `.env` file is not copied into the image. Compose always
sets the container's `DATA_DIR` to `/app/data`, regardless of the local value.

The response directory is mounted read-only. After editing its files, reload
them by restarting the bot (no rebuild needed):

```bash
docker compose restart oracle
```

After changing Rust code:

```bash
docker compose up --build -d
```

After changing environment settings, recreate the container:

```bash
docker compose up -d --force-recreate
```

Stop and remove the container:

```bash
docker compose down
```

Stop any locally running copy before starting the container with the same bot
token. No inbound ports need to be published.

The runtime uses a non-root user and a read-only filesystem, drops Linux
capabilities, and rotates container logs. The `Z` mount option supports
SELinux hosts such as Fedora by privately relabeling the mounted `data/`
directory. The files must be readable by the container user.

### Podman

On a machine with Podman and podman-compose instead of Docker:

```bash
podman-compose up --build -d
podman-compose logs -f oracle
podman-compose restart oracle
podman-compose down
```

## Build and publish to Docker Hub

The scripts in `bin/` default to Podman, version `0.1.0`, and the repository
`docker.io/robaldwin/oracle`. They work from any working directory.
`build.sh` and `publish.sh` default to the local system architecture.

Log in interactively first, using your Docker Hub username and an access token:

```bash
podman login docker.io
```

Build for your local system, then publish only after the build succeeds:

```bash
./bin/build.sh robaldwin 0.1.0
./bin/publish.sh robaldwin 0.1.0
```

On an x86-64 system these commands use
`docker.io/robaldwin/oracle:0.1.0-amd64`; on an ARM64 system they use
`docker.io/robaldwin/oracle:0.1.0-arm64`. Publishing checks that the local image
exists and has the requested architecture. It does not rebuild, log in, or
deploy the bot.

For an ARM64 image regardless of your local architecture:

```bash
./bin/buildarm64.sh robaldwin 0.1.0
./bin/publisharm64.sh robaldwin 0.1.0
```

Username and version are optional in all four scripts, defaulting to
`robaldwin` and `0.1.0`. The username is your Docker Hub username, not your
email or password. For example, `./bin/buildarm64.sh anotheruser 0.2.0`
builds `docker.io/anotheruser/oracle:0.2.0-arm64`.

The general scripts also accept an explicit architecture as their third
argument, such as `./bin/build.sh robaldwin 0.1.0 amd64`.

Cross-architecture builds require QEMU/binfmt. On Fedora, install and enable
it with:

```bash
sudo dnf install qemu-user-static
sudo systemctl restart systemd-binfmt
```

Emulated Rust builds can be much slower than native builds.

You can override the engine or repository:

```bash
CONTAINER_ENGINE=docker IMAGE_REPOSITORY=docker.io/YOUR_USERNAME/oracle \
    ./bin/build.sh "" 0.1.0 arm64
```

An explicit username argument takes precedence over `IMAGE_REPOSITORY`.
Pass an empty username as above to use the repository override.

Use the same overrides when running `publish.sh`, and authenticate using the
same container engine. Run any script with `--help` for usage. The helper
`image-config.sh` is sourced by the scripts, not run directly.

On the remote server, remove the Compose `build` section and set:

```yaml
image: docker.io/robaldwin/oracle:0.1.0-arm64
```

Then deploy:

```bash
docker compose pull
docker compose up -d
```

The `.env` and `data/` files remain separate from the published image.

## Tests

```bash
cargo test
```

The local `.env` parsing test requires a `.env` file, but does not connect to
Discord. The image build compiles the application without running tests or
starting the bot.
