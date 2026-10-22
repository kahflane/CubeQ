# CubeQ CLI Reference

Curated operational reference for common `cubeq` commands. It is not an exhaustive command tree; use `cubeq --help` and `cubeq <command> --help` for the commands and flags in the installed build.

## Table of Contents

1. [Agent](#agent)
2. [Onboarding](#onboarding)
3. [Status & Diagnostics](#status--diagnostics)
4. [Memory](#memory)
5. [Cron](#cron)
6. [Providers & Models](#providers--models)
7. [Gateway & Daemon](#gateway--daemon)
8. [Service Management](#service-management)
9. [Channels](#channels)
10. [Security & Emergency Stop](#security--emergency-stop)
11. [Hardware Peripherals](#hardware-peripherals)
12. [Skills](#skills)
13. [Shell Completions](#shell-completions)

---

## Agent

Interactive chat or single-message mode.

```bash
cubeq agent -a assistant                                          # Interactive REPL
cubeq agent -a assistant -m "Summarize today's logs"              # Single message
cubeq agent -a assistant -p anthropic --model claude-sonnet-4-6   # Override provider/model
cubeq agent -a assistant -t 0.3                                   # Set temperature
cubeq agent -a assistant --peripheral nucleo-f401re:/dev/ttyACM0  # Attach hardware
```

`-a <alias>` is required and must match a configured `[agents.<alias>]` entry — there is no default agent.

**Key flags:**
- `-m <message>` — single message mode (no REPL)
- `-p <provider>` — override provider (openrouter, anthropic, openai, ollama)
- `--model <model>` — override model
- `-t <float>` — temperature (0.0–2.0)
- `--peripheral <name>:<port>` — attach hardware peripheral

The agent has access to 30+ tools gated by security policy: shell, file_read, file_write, file_edit, glob_search, content_search, memory_store, memory_recall, memory_forget, browser, http_request, web_fetch, web_search, cron, delegate, git, and more. Max tool iterations defaults to 10.

---

## Quickstart

First-time setup. Picks a model provider, writes a working `~/.cubeq/config.toml`, and binds one agent to that provider in a single shot.

```bash
cubeq quickstart                                                           # Interactive prompts
cubeq quickstart --model-provider ollama --model qwen2.5:7b                # Non-interactive (local)
cubeq quickstart --model-provider openrouter --model openrouter/auto \
                    --api-key sk-or-... --agent or                            # Non-interactive (hosted)
```

**Flags:**
- `--model-provider <name>` — anthropic, openai, openrouter, ollama, gemini, glm, telnyx, …
- `--model <id>` — model id to write for the new provider entry
- `--api-key <key>` — API key (omit for local providers like ollama)
- `--agent <alias>` — agent alias (defaults to a sanitized provider name)

Creates `~/.cubeq/config.toml` with `0600` permissions. Quickstart is idempotent: re-running it on a configured install leaves the existing config alone. To change one field afterward, use `cubeq config set <path> <value>`; to reconfigure channels, use `cubeq config set channels.<type>.<alias>.<field> <value>`. For secret fields, omit the value to use the masked input prompt, for example `cubeq config set channels.telegram.default.bot-token`. Per-channel guides live under [Channels → Overview](../../../../docs/book/src/channels/overview.md).

---

## Status & Diagnostics

```bash
cubeq status                    # System overview
cubeq doctor                    # Run all diagnostic checks
cubeq doctor models             # Probe model connectivity
cubeq doctor traces             # Query execution traces
```

---

## Memory

```bash
cubeq memory list                              # List all entries
cubeq memory list --category core --limit 10   # Filtered list
cubeq memory get "some-key"                    # Get specific entry
cubeq memory stats                             # Usage statistics
cubeq memory clear --key "prefix" --yes        # Delete entries (requires --yes)
```

**Key flags:**
- `--category <name>` — filter by category (core, daily, conversation, custom)
- `--limit <n>` — limit results
- `--key <prefix>` — key prefix for clear operations
- `--yes` — skip confirmation (required for clear)

---

## Cron

```bash
cubeq cron list                                                      # List all jobs
cubeq cron add '0 9 * * 1-5' 'Good morning' --tz America/New_York   # Recurring (cron expr)
cubeq cron add-at '2026-03-11T10:00:00Z' 'Remind me about meeting'  # One-time at specific time
cubeq cron add-every 3600000 'Check server health'                   # Interval in milliseconds
cubeq cron once 30m 'Follow up on that task'                         # Delay from now
cubeq cron pause <id>                                                # Pause job
cubeq cron resume <id>                                               # Resume job
cubeq cron remove <id>                                               # Delete job
```

**Subcommands:**
- `add <cron-expr> <command>` — standard cron expression (5-field)
- `add-at <iso-datetime> <command>` — fire once at exact time
- `add-every <ms> <command>` — repeating interval
- `once <duration> <command>` — delay from now (e.g., `30m`, `2h`, `1d`)

---

## Providers & Models

```bash
cubeq providers                                # List all 40+ supported providers
cubeq models list                              # Show cached model catalog
cubeq models refresh --all                     # Refresh catalogs from all providers
cubeq models set anthropic/claude-sonnet-4-6   # Set default model
cubeq models status                            # Current model info
```

Model routing in config.toml:

```toml
[[model_routes]]
hint = "reasoning"
model_provider = "openrouter"
model = "anthropic/claude-sonnet-4-6"
```

---

## Gateway & Daemon

```bash
cubeq gateway                                 # Start HTTP gateway (foreground)
cubeq gateway -p 8080 --host 127.0.0.1        # Custom port/host

cubeq daemon                                  # Gateway + channels + scheduler + heartbeat
cubeq daemon -p 8080 --host 0.0.0.0           # Custom bind
```

**Gateway defaults:**
- Port: 42617
- Host: 127.0.0.1
- Pairing required: true
- Public bind allowed: false

---

## Service Management

OS service lifecycle (systemd on Linux, launchd on macOS).

```bash
cubeq service install     # Install as system service
cubeq service start       # Start the service
cubeq service status      # Check service status
cubeq service stop        # Stop the service
cubeq service restart     # Restart the service
cubeq service uninstall   # Remove the service
```

**Logs:**
- macOS: `~/.cubeq/logs/daemon.stdout.log`
- Linux: `journalctl -u cubeq`

---

## Channels

Channels use alias-keyed entries under `[channels.<type>.<alias>]`. Availability depends on the installed build's feature set; consult the current channel guide and generated config reference before configuring a channel.

```bash
cubeq channels list       # List configured channels
cubeq channel doctor      # Check channel health
```

---

## Security & Emergency Stop

```bash
cubeq estop --level kill-all                              # Stop everything
cubeq estop --level network-kill                          # Block all network access
cubeq estop --level domain-block --domain "*.example.com" # Block specific domains
cubeq estop --level tool-freeze --tool shell              # Freeze specific tool
cubeq estop status                                        # Check estop state
cubeq estop resume --network                              # Resume (may require OTP)
```

**Estop levels:**
- `kill-all` — nuclear option, stops all agent activity
- `network-kill` — blocks all outbound network
- `domain-block` — blocks specific domain patterns
- `tool-freeze` — freezes individual tools

Autonomy lives on a risk profile and a runtime profile, both alias-keyed; the agent points at them via `[agents.<alias>] risk_profile = "..."` and `runtime_profile = "..."`:

```toml
[risk_profiles.assistant]
level = "supervised"                           # readonly | supervised | full
workspace_only = true
allowed_commands = ["git", "cargo", "python"]
forbidden_paths = ["/etc", "/root", "~/.ssh"]

[runtime_profiles.assistant]
max_actions_per_hour = 20
max_cost_per_day_cents = 500
```

---

## Hardware Peripherals

```bash
cubeq hardware discover                              # Find USB devices
cubeq hardware introspect /dev/ttyACM0               # Probe device capabilities
cubeq peripheral list                                # List configured peripherals
cubeq peripheral add nucleo-f401re /dev/ttyACM0      # Add peripheral
cubeq peripheral flash-nucleo                        # Flash STM32 firmware
cubeq peripheral flash --port /dev/cu.usbmodem101    # Flash Arduino firmware
```

**Supported boards:** STM32 Nucleo-F401RE, Arduino Uno R4, Raspberry Pi GPIO, ESP32.

Attach to agent session: `cubeq agent -a assistant --peripheral nucleo-f401re:/dev/ttyACM0`

---

## Skills

```bash
cubeq skills list         # List installed skills
cubeq skills install <path-or-url>  # Install a skill
cubeq skills audit        # Audit installed skills
cubeq skills remove <name>  # Remove a skill
```

---

## Shell Completions

```bash
cubeq completions zsh     # Generate Zsh completions
cubeq completions bash    # Generate Bash completions
cubeq completions fish    # Generate Fish completions
```

---

## Config File

Default location: `~/.cubeq/config.toml`

Config resolution order (first match wins):
1. `CUBEQ_CONFIG_DIR` environment variable
2. `CUBEQ_WORKSPACE` environment variable
3. `~/.cubeq/active_workspace.toml` marker file
4. `~/.cubeq/config.toml` (default)
