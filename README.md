# CubeQ

CubeQ is a self-hosted AI agent runtime. It is a single Rust binary that you
configure with one TOML file and run on your own machine.

It talks to LLM providers (Anthropic, OpenAI, Ollama and other
OpenAI-compatible endpoints), reaches you through chat channels, runs tools
under a security policy, and keeps long-term memory.

## Features

- **Providers**: Anthropic, OpenAI, Ollama, OpenRouter and any
  OpenAI-compatible endpoint, with fallback and routing.
- **Channels**: Telegram, Discord, email, webhooks, the filesystem and ACP by
  default. More channels (Slack, Matrix, WhatsApp and others) are behind
  feature flags.
- **Tools**: shell, files, web fetch and search, HTTP, git, browser and more,
  all gated by an autonomy level and risk profiles.
- **Security**: supervised autonomy by default, workspace boundaries, command
  policy, OS sandboxes (Landlock, Bubblewrap, Seatbelt), approvals and an
  emergency stop.
- **Memory**: SQLite by default, with embeddings, consolidation and retrieval.
  Postgres is optional.
- **Gateway**: HTTP and WebSocket API for clients and webhooks.
- **SOPs and cron**: event-triggered standard operating procedures with
  approval gates, plus scheduled tasks.
- **Plugins and skills**: WASM plugins and file-based skills.
- **Hardware**: GPIO, I2C, SPI and USB peripherals (behind the `hardware`
  feature).

## Requirements

- Rust 1.96 or newer (`rustup update stable`)
- A C toolchain for some dependencies (Xcode Command Line Tools on macOS,
  `build-essential` on Debian/Ubuntu)

## Build

```bash
git clone https://github.com/kahflane/CubeQ.git
cd CubeQ
cargo build --release
./target/release/cubeq --help
```

To install the binary into `~/.cargo/bin`:

```bash
cargo install --path . --locked
```

### Feature flags

The default build includes the agent runtime, the gateway, ACP, Prometheus
metrics and the default channels. Add more with `--features`:

```bash
cargo build --release --features "plugins-wasm,hardware,memory-postgres"
```

`Cargo.toml` lists every feature. The `embedded-web` feature needs a web
dashboard build, which this repository does not include.

## Quick start

```bash
cubeq quickstart           # pick a provider and write a working config
cubeq agent -a <alias>     # chat with the agent named <alias>
cubeq daemon               # run the long-lived daemon (channels, cron, SOPs)
cubeq service install      # register as a launchd/systemd user service
cubeq service start
```

Check your setup with `cubeq doctor` and `cubeq status`.

## Configuration

CubeQ reads one TOML file at `~/.cubeq/config.toml`. Set `CUBEQ_CONFIG_DIR`
to use another directory.

A minimal config has a provider, an agent that uses it, and a risk profile:

```toml
schema_version = 3

[providers.models.anthropic.default]
model = "claude-sonnet-4-5"

[agents.default]
model_provider = "anthropic.default"
risk_profile = "default"
runtime_profile = "default"

[risk_profiles.default]
level = "supervised"

[runtime_profiles.default]
```

`cubeq quickstart` writes this for you. Use `cubeq config` to read and change
values, and `cubeq config schema` to print the full JSON Schema.

### Plugin and skill registries

CubeQ has no built-in registry. To install plugins or skills by name, point it
at your own:

- Plugins: pass `--registry <url>` to `cubeq plugin install` or
  `cubeq plugin search`, or set `CUBEQ_PLUGIN_REGISTRY_URL`.
- Skills: set `skills.registry_url` in the config.

Installing from a local path works without a registry.

## Commands

| Command | What it does |
| --- | --- |
| `quickstart` | Create your first agent end to end |
| `agent` | Start the interactive agent loop |
| `daemon` | Run the long-lived daemon |
| `gateway` | Manage the HTTP/WebSocket gateway |
| `service` | Install and control the OS service |
| `status`, `doctor`, `self-test` | Inspect and diagnose the install |
| `update` | Update to the latest GitHub release |
| `config` | Read, set and migrate configuration |
| `providers`, `models`, `auth` | Manage LLM providers and credentials |
| `channel`, `channels` | Manage chat channels |
| `agents` | Manage agent aliases |
| `memory` | Manage agent memory |
| `skills`, `plugin` | Manage skills and WASM plugins |
| `sop`, `cron` | Manage SOPs and scheduled tasks |
| `security`, `estop` | Inspect security posture and use the emergency stop |
| `hardware`, `peripheral` | Discover and manage hardware |
| `acp` | Run the ACP server over stdio |
| `eval` | Run the agent evaluation harness |
| `completions` | Generate shell completions |

Run `cubeq <command> --help` for details.

## Updating

`cubeq update` installs the latest release from
[GitHub releases](https://github.com/kahflane/CubeQ/releases), with backup
and rollback. Use `cubeq update --check` to only check.

A release must attach one archive per platform, named
`cubeq-<target>.tar.gz` (`.zip` on Windows), for example
`cubeq-aarch64-apple-darwin.tar.gz`. Attach a `SHA256SUMS` file to have
downloads verified.

## Project layout

```
src/                     the `cubeq` CLI binary and library
crates/
  cubeq-api              shared traits and types
  cubeq-config           config schema, secrets and migration
  cubeq-runtime          agent loop, security, cron, SOPs, skills, RPC
  cubeq-providers        LLM providers and auth
  cubeq-channels         messaging channel integrations
  cubeq-tools            agent-callable tools
  cubeq-memory           memory backends and retrieval
  cubeq-gateway          HTTP/WebSocket gateway
  cubeq-plugins          WASM plugin host
  cubeq-hardware         hardware discovery and peripherals
  cubeq-eval             evaluation harness
  cubeq-log              structured logging
  cubeq-tls              TLS and mTLS helpers
  ...                    smaller support crates
wit/                     WIT interfaces for WASM plugins
vendor/                  patched third-party crates
```

## Development

```bash
cargo check --workspace --all-targets
cargo test --workspace
cargo fmt --all
cargo clippy --workspace --all-targets
```

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE), at
your option. See [NOTICE](NOTICE) for attribution.
