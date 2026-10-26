# TTYME

> A high-performance, asynchronous terminal-based chat system written in Rust featuring a Tokio TCP broadcast backend and a retro Cursive TUI client.

[![Rust](https://img.shields.io/badge/Rust-2024%20Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![Async](https://img.shields.io/badge/Async-Tokio-blue?logo=tokio)](https://tokio.rs/)
[![TUI](https://img.shields.io/badge/TUI-Cursive-brightgreen)](https://github.com/gyscos/cursive)
[![License](https://img.shields.io/badge/License-MIT-lightgrey.svg)](LICENSE)

TTYME is a modern client-server terminal chat application designed for low-latency command-line communication. It pairs a concurrent asynchronous broadcast TCP daemon (`backend`) with a nostalgic, responsive terminal user interface (`app`) styled with retro borders, drop shadows, and automatic message scrolling.

---

## Table of Contents

- [Key Features](#key-features)
- [Tech Stack](#tech-stack)
- [Prerequisites](#prerequisites)
- [Getting Started](#getting-started)
  - [1. Clone the Repository](#1-clone-the-repository)
  - [2. Build the Project](#2-build-the-project)
  - [3. Start the Server Daemon](#3-start-the-server-daemon)
  - [4. Launch Chat Clients](#4-launch-chat-clients)
- [Architecture](#architecture)
  - [Directory Tree](#directory-tree)
  - [System Flow & Data Lifecycle](#system-flow--data-lifecycle)
  - [Wire Protocol & Data Models](#wire-protocol--data-models)
  - [Core Components Breakdown](#core-components-breakdown)
- [Client Commands & Keybindings](#client-commands--keybindings)
- [Configuration](#configuration)
- [Available Cargo Commands](#available-cargo-commands)
- [Testing](#testing)
- [Production Deployment](#production-deployment)
  - [Standalone Release Binaries](#standalone-release-binaries)
  - [Systemd Service Setup (Linux VPS)](#systemd-service-setup-linux-vps)
  - [Docker Containerization](#docker-containerization)
- [Troubleshooting](#troubleshooting)
- [Contributing](#contributing)
- [License](#license)

---

## Key Features

- **Concurrent Async Networking**: Built on top of `tokio` asynchronous TCP streams with zero-blocking I/O and lightweight task scheduling.
- **Broadcast Architecture**: Utilizes `tokio::sync::broadcast` multi-producer, multi-consumer channels to fan out messages effortlessly to all active subscribers.
- **Interactive Retro TUI**: Designed using `cursive` with custom retro color palettes, drop shadows, panel framing, and automatic stick-to-bottom scroll management.
- **Thread-Safe Cross-Task Communication**: Coordinates background socket reading and UI event dispatching via thread-safe `CbSink` channels and `Arc<Mutex<OwnedWriteHalf>>`.
- **System Events & Handshake**: Dynamic join/leave system notifications with microsecond-accurate timestamps powered by `chrono`.
- **In-Chat Commands**: Built-in interactive slash commands (`/help`, `/clear`, `/quit`) and intuitive keyboard shortcuts.
- **Robust JSON Serialization**: Structured message payloads serialized and deserialized using `serde` and `serde_json`.

---

## Tech Stack

| Layer | Technology | Version / Specification | Purpose |
| :--- | :--- | :--- | :--- |
| **Language** | [Rust](https://www.rust-lang.org/) | Edition 2024 | Memory safety, speed, zero-cost abstractions |
| **Async Runtime** | [Tokio](https://tokio.rs/) | `1.54` (features: `full`) | Event loop, non-blocking TCP sockets, tasks |
| **Terminal UI** | [Cursive](https://github.com/gyscos/cursive) | `0.21` | Retro terminal views, event hooks, layout engine |
| **Serialization** | [Serde / Serde JSON](https://serde.rs/) | `1.0` | Strongly typed JSON message wire protocol |
| **Date & Time** | [Chrono](https://github.com/chronotope/chrono) | `0.4` | Human-readable timestamps for events |

---

## Prerequisites

Before running TTYME, make sure you have the following installed on your machine:

- **Rust Toolchain**: Rust compiler `rustc` and package manager `cargo` (compatible with 2024 edition).
  Install via [rustup](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source "$HOME/.cargo/env"
  ```
- **Terminal Emulator**: Any modern ANSI/UTF-8 terminal emulator (e.g., Alacritty, Kitty, WezTerm, iTerm2, GNOME Terminal, Windows Terminal).
- **C Libraries (Optional on Linux)**: Standard `ncurses` or backend libraries used by Cursive if building with alternate backends:
  ```bash
  # Debian / Ubuntu
  sudo apt-get update && sudo apt-get install -y build-essential libncursesw5-dev
  
  # Fedora / RHEL
  sudo dnf install -y ncurses-devel
  
  # Arch Linux
  sudo pacman -S ncurses base-devel
  ```

---

## Getting Started

### 1. Clone the Repository

```bash
git clone https://github.com/MostafaSensei106/TTYME.git
cd TTYME
```

### 2. Build the Project

Verify your environment by compiling all targets:

```bash
cargo build
```

---

### 3. Start the Server Daemon

Launch the server listener in your first terminal session:

```bash
cargo run --bin backend
```

Upon launching, the server binds to `127.0.0.1:8080` and displays its startup banner:

```text
╔══════════════════════════════════════════╗
║                                          ║
║  ████████╗████████╗██╗   ██╗███╗   ███╗  ║
║  ╚══██╔══╝╚══██╔══╝╚██╗ ██╔╝████╗ ████║  ║
║     ██║      ██║    ╚████╔╝ ██╔████╔██║  ║
║     ██║      ██║     ╚██╔╝  ██║╚██╔╝██║  ║
║     ██║      ██║      ██║   ██║ ╚═╝ ██║  ║
║     ╚═╝      ╚═╝      ╚═╝   ╚═╝     ╚═╝  ║
║                                          ║
╚══════════════════════════════════════════╝

    TTYME server started successfully
    Listening on: 127.0.0.1:8080

    Press Ctrl+C to stop the server.
```

---

### 4. Launch Chat Clients

Open one or more separate terminal windows to join the chat. Provide your desired username as a command-line argument:

#### Terminal Client 1 (Alice):
```bash
cargo run --bin app -- Alice
```

#### Terminal Client 2 (Bob):
```bash
cargo run --bin app -- Bob
```

The server console immediately registers each incoming client and logs connection activity:

```text
┌─[14:40:12] New Connection
└─ Address: 127.0.0.1:54320

┌─[14:40:19] New Connection
└─ Address: 127.0.0.1:54322
```

Both clients will see system notifications (e.g., `[14:40:12] [System] Alice Joined the chat`) and can exchange live messages in real time!

---

## Architecture

TTYME is organized as a Cargo workspace with a shared core library (`ttyme`) and two discrete binary targets (`backend` and `app`).

### Directory Tree

```
TTYME/
├── Cargo.toml                  # Workspace dependencies & package metadata
├── Cargo.lock                  # Dependency lockfile
├── README.md                   # Project documentation
├── src/
│   ├── lib.rs                  # Library entry point exposing core modules
│   ├── core/                   # Shared crate modules (models, config, I/O)
│   │   ├── mod.rs              # Re-exports constants, io, and models
│   │   ├── constants/
│   │   │   ├── mod.rs
│   │   │   └── config.rs       # Constants struct (default host & port)
│   │   ├── io/
│   │   │   ├── mod.rs
│   │   │   └── io_handler.rs   # Async TCP connection & broadcast handler
│   │   └── models/
│   │       ├── mod.rs
│   │       └── chat_message_model.rs  # ChatMessage struct & MessageType enum
│   └── bin/
│       ├── backend/
│       │   └── main.rs         # TCP server daemon entry point
│       └── app/
│           ├── main.rs         # TUI application entry point & orchestration
│           ├── network/
│           │   ├── mod.rs      # Network client module definitions
│           │   └── client.rs   # TCP connection, handshake, & async reader task
│           └── ui/
│               ├── mod.rs      # UI components & styling exports
│               ├── handlers.rs # Input command routing & message dispatching
│               ├── layout.rs   # Cursive view construction & window hierarchy
│               ├── message.rs  # StyledString rich message formatting
│               └── theme.rs    # Retro color palette & border styling
```

---

### System Flow & Data Lifecycle

```
+-------------------------------------------------------------------------------+
|                                  TTYME Architecture                           |
+-------------------------------------------------------------------------------+

  +-----------------------+                    +-----------------------+
  |   Client A (Alice)    |                    |    Client B (Bob)     |
  |  [Cursive TUI / App]  |                    |  [Cursive TUI / App]  |
  +-----------+-----------+                    +-----------+-----------+
              |                                            ^
  (1) "Hello" | TCP Socket                      TCP Socket | (4) JSON Payload
              v                                            |
      +-------+--------------------------------------------+-------+
      |                       TTYME Server                         |
      |                 [src/bin/backend/main.rs]                  |
      |                                                            |
      |  1. listener.accept() -> tokio::spawn(handle_connection)   |
      |  2. Read line -> Parse text                                |
      |  3. Construct ChatMessage {                                |
      |        user_name: "Alice",                                 |
      |        message_content: "Hello",                           |
      |        message_type: UserMessage,                          |
      |        timestamp: "14:42:01"                               |
      |     }                                                      |
      |  4. Serialize -> JSON string                               |
      |  5. broadcast::Sender::send(json_string)                   |
      +----------------------------+-------------------------------+
                                   |
                                   v
             +-------------------------------------------+
             |         tokio::sync::broadcast            |
             |       Channel Capacity: 50 Items          |
             +-------------------------------------------+
                                   |
         +-------------------------+-------------------------+
         | (Delivered to all active subscribers)             |
         v                                                   v
  +---------------+                                   +---------------+
  |  Receiver A   |                                   |  Receiver B   |
  | (Alice's rx)  |                                   |  (Bob's rx)   |
  +-------+-------+                                   +-------+-------+
          |                                                   |
          v                                                   v
   TCP Stream Write                                    TCP Stream Write
          |                                                   |
          v                                                   v
  Client A UI Screen                                  Client B UI Screen
  "- [14:42:01]                                       "- [14:42:01]
   -> Alice > Hello"                                   -> Alice > Hello"
```

---

### Wire Protocol & Data Models

Communication over the TCP stream uses newline-delimited JSON (`\n`).

#### 1. Connection Handshake
Immediately upon establishing a TCP connection, the client sends a single newline-terminated string containing the username:
```text
Alice\n
```

#### 2. Message Payload Schema
Every event broadcast by the server is formatted as a JSON object adhering to [`ChatMessage`](file:///home/ottafa/Devolpments/TTYME/src/core/models/chat_message_model.rs):

```json
{
  "user_name": "Alice",
  "message_content": "Joined the chat",
  "message_type": "SystemNotification",
  "timestamp": "06:40:12 PM"
}
```

```json
{
  "user_name": "Alice",
  "message_content": "Hey everyone, welcome to TTYME!",
  "message_type": "UserMessage",
  "timestamp": "06:40:35 PM"
}
```

#### 3. Data Definitions (`src/core/models/chat_message_model.rs`)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub user_name: String,
    pub message_content: String,
    pub message_type: MessageType,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    UserMessage,
    SystemNotification,
}
```

---

### Core Components Breakdown

#### `backend` (`src/bin/backend/main.rs`)
- Binds a non-blocking `TcpListener` to the address resolved from `Constants`.
- Creates a `tokio::sync::broadcast::channel::<String>(50)` channel.
- Dispatches each accepted connection to `handle_connection(socket, tx, rx)` inside a green thread (`tokio::spawn`).

#### `io_handler` (`src/core/io/io_handler.rs`)
- Splits socket into async reader and writer.
- Performs username handshake; closes connection if username is empty.
- Broadcasts a `SystemNotification` announcing the client joined.
- Runs an event loop utilizing `tokio::select!`:
  - Receives lines sent by the connected client and broadcasts them as `UserMessage`.
  - Listens to the broadcast channel receiver `rx` and writes incoming serialized messages out to the client's TCP writer.
- Automatically broadcasts a `"Left the chat"` notification upon socket EOF or disconnect.

#### `app` (`src/bin/app/main.rs`)
- Instantiates the `Cursive` terminal environment with a custom retro theme.
- Splits layout into header, scrollable message panel, input box, and quick-help footer.
- Spawns a background Tokio reader task that decodes incoming JSON messages and uses `siv.cb_sink().send(...)` to thread-safely append formatted text to the active TUI.
- Handles text input asynchronously, routing slash commands locally (`/help`, `/clear`, `/quit`) or dispatching chat messages through `Arc<Mutex<OwnedWriteHalf>>`.

---

## Client Commands & Keybindings

| Key / Command | Context | Action |
| :--- | :--- | :--- |
| `Enter` | Input Box | Sends the current message buffer or executes the command |
| `Esc` | Anywhere | Quits the TTYME application immediately |
| `/` | Anywhere | Automatically places focus into the input box and begins a command |
| `/help` | In Chat Input | Prints complete command reference with descriptions |
| `/clear` | In Chat Input | Clears all current messages from your message window |
| `/whoami` | In Chat Input | Displays user profile, host, and connection status |
| `/time` | In Chat Input | Displays the current local date and 12-hour AM/PM time |
| `/ping` | In Chat Input | Verifies local server connection responsiveness |
| `/shrug` | In Chat Input | Sends ASCII shrug emoticon: `¯\_(ツ)_/¯` |
| `/tableflip` | In Chat Input | Sends ASCII tableflip emoticon: `(╯°□°)╯︵ ┻━┻` |
| `/unflip` | In Chat Input | Sends ASCII unflip emoticon: `┬─┬ノ( º _ ºノ)` |
| `/eggplant` | In Chat Input | Sends ASCII eggplant emoticon: `(づ｡◕‿‿◕｡)づ [Eggplant] ~-(:::)` |
| `/cucumber`, `/pickle` | In Chat Input | Sends ASCII cucumber/pickle emote |
| `/gigachad`, `/chad` | In Chat Input | Sends Gigachad ASCII emote |
| `/rip`, `/f` | In Chat Input | Sends `[RIP] Press F to pay respects` |
| `/gg` | In Chat Input | Sends `[GG] Good game, well played!` |
| `/ez` | In Chat Input | Sends `[EZ] Easy peasy, lemon squeezy!` |
| `/noob` | In Chat Input | Sends `[NOOB] Skill issue detected!` |
| `/sus` | In Chat Input | Sends Among Us ASCII emote `ඞ [SUS]` |
| `/doge` | In Chat Input | Sends Doge ASCII emote `(◕ᴥ◕)` |
| `/cat` | In Chat Input | Sends Cat ASCII emote `(=^･ω･^=)` |
| `/dance` | In Chat Input | Sends Dancing ASCII emote |
| `/flex` | In Chat Input | Sends Flexing ASCII emote `ᕙ(⇀‸↼‶)ᕗ` |
| `/popcorn` | In Chat Input | Sends Popcorn eating ASCII emote `( ˘▽˘)っ♨` |
| `/me <action>` | In Chat Input | Broadcasts an IRC-style action line (`* User action`) |
| `/quit`, `/exit` | In Chat Input | Gracefully exits the application |

---

## Configuration

Default networking configuration is declared in [`src/core/constants/config.rs`](file:///home/ottafa/Devolpments/TTYME/src/core/constants/config.rs):

```rust
pub struct Constants {
    pub host: &'static str,
    pub port: u16,
}

impl Default for Constants {
    fn default() -> Self {
        Self {
            host: "127.0.0.1",
            port: 8080,
        }
    }
}
```

To configure custom binding parameters or ports for custom environments:
1. Modify `host` or `port` defaults in `Constants::default()`.
2. Recompile using `cargo build --release`.

---

## Available Cargo Commands

| Command | Description |
| :--- | :--- |
| `cargo build` | Compiles the library and all binaries in debug mode |
| `cargo build --release` | Produces optimized production binaries in `target/release/` |
| `cargo check` | Fast syntax and type verification across all targets |
| `cargo check --lib` | Checks only the shared core library target |
| `cargo check --bin backend` | Checks only the server daemon |
| `cargo check --bin app` | Checks only the Cursive TUI client |
| `cargo run --bin backend` | Starts the TCP broadcast server daemon |
| `cargo run --bin app -- <name>` | Launches a client session with the specified username |
| `cargo test` | Runs the test suite |
| `cargo clean` | Removes compiled artifacts from `target/` |

---

## Testing

TTYME contains automated unit and integration tests.

### Running Tests

Run all unit tests across the workspace:
```bash
cargo test
```

Run tests with verbose output:
```bash
cargo test -- --nocapture
```

### Adding New Tests

Unit tests for data models and wire protocols can be added in `src/core/models/chat_message_model.rs` or in dedicated `tests/` integration suites:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_message_serialization() {
        let msg = ChatMessage {
            user_name: "Developer".to_string(),
            message_content: "Ping".to_string(),
            message_type: MessageType::UserMessage,
            timestamp: "12:00:00".to_string(),
        };

        let serialized = serde_json::to_string(&msg).expect("Failed to serialize");
        let deserialized: ChatMessage = serde_json::from_str(&serialized).expect("Failed to deserialize");

        assert_eq!(deserialized.user_name, "Developer");
        assert_eq!(deserialized.message_content, "Ping");
    }
}
```

---

## Production Deployment

### Standalone Release Binaries

To deploy TTYME as lightweight, native binaries without Cargo installed on the target machine:

```bash
cargo build --release
```

The resulting standalone executables will be located in:
- `target/release/backend` (Server daemon)
- `target/release/app` (Client application)

Transfer the `backend` binary to your server:
```bash
scp target/release/backend user@your-server-ip:/usr/local/bin/ttyme-server
```

---

### Systemd Service Setup (Linux VPS)

To run the `backend` server continuously as a background service managed by `systemd`:

1. Copy the binary to `/usr/local/bin`:
   ```bash
   sudo cp target/release/backend /usr/local/bin/ttyme-server
   sudo chmod +x /usr/local/bin/ttyme-server
   ```

2. Create a dedicated system user:
   ```bash
   sudo useradd -r -s /bin/false ttyme
   ```

3. Create the service definition file at `/etc/systemd/system/ttyme.service`:
   ```ini
   [Unit]
   Description=TTYME Broadcast Chat Server
   After=network.target

   [Service]
   Type=simple
   User=ttyme
   ExecStart=/usr/local/bin/ttyme-server
   Restart=always
   RestartSec=5
   LimitNOFILE=65535

   [Install]
   WantedBy=multi-user.target
   ```

4. Reload systemd, enable, and start the service:
   ```bash
   sudo systemctl daemon-reload
   sudo systemctl enable --now ttyme
   ```

5. Monitor server status and connection logs:
   ```bash
   sudo systemctl status ttyme
   sudo journalctl -u ttyme -f
   ```

---

### Docker Containerization

Deploy the backend using Docker with this multi-stage build:

#### `Dockerfile` (for Backend)
```dockerfile
# Build Stage
FROM rust:1-bullseye AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin backend

# Runtime Stage
FROM debian:bullseye-slim
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/backend /usr/local/bin/ttyme-server

EXPOSE 8080
USER 1000:1000

CMD ["ttyme-server"]
```

#### Build & Run Container:
```bash
# Build Docker image
docker build -t ttyme-backend:latest .

# Run container exposing port 8080
docker run -d \
  --name ttyme-server \
  --restart unless-stopped \
  -p 8080:8080 \
  ttyme-backend:latest
```

---

## Troubleshooting

### 1. `Address already in use (os error 98)`
- **Cause**: Another instance of `backend` or another process is already bound to port `8080`.
- **Solution**:
  Find and terminate the process holding the port:
  ```bash
  # Locate PID
  lsof -i :8080
  # Or with ss
  ss -lptn 'sport = :8080'
  
  # Terminate process
  kill -9 <PID>
  ```

### 2. `Connection refused (os error 111)` on Client Launch
- **Cause**: The client cannot reach the server at `127.0.0.1:8080`.
- **Solution**:
  1. Ensure the `backend` binary is running:
     ```bash
     pgrep -l backend
     ```
  2. If connecting to a remote server, verify firewall permissions allow traffic on port `8080`:
     ```bash
     sudo ufw allow 8080/tcp
     ```

### 3. `Please provide a username as an argument`
- **Cause**: Running `cargo run --bin app` without passing a username.
- **Solution**:
  Pass your username after `--`:
  ```bash
  cargo run --bin app -- <your_username>
  ```

### 4. Terminal Visual Glitches / Layout Distortion
- **Cause**: Terminal window dimensions smaller than the minimum layout bounds or unsupported terminal control sequences.
- **Solution**:
  - Enlarge your terminal window (minimum suggested size: 80 columns x 24 rows).
  - Verify your `TERM` environment variable supports modern ANSI colors:
    ```bash
    export TERM=xterm-256color
    ```

---

## Contributing

Contributions, feedback, and pull requests are warmly welcome!

1. **Fork the Repository**: Click "Fork" on GitHub.
2. **Create a Feature Branch**:
   ```bash
   git checkout -b feature/amazing-feature
   ```
3. **Commit Your Changes**:
   ```bash
   git commit -m "Add amazing feature"
   ```
4. **Push to Your Branch**:
   ```bash
   git push origin feature/amazing-feature
   ```
5. **Open a Pull Request**: Detail your changes and submit for review.

---

## License

This project is open-source under the [MIT License](LICENSE). Feel free to use, modify, and distribute it in your own projects.
