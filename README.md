# Orb — simple, hands-on LAN file transfer (learning project)

Orb is an experimental, educational Rust project that implements a minimal peer-to-peer file transfer over a local network. It was built to explore practical networking concepts (UDP discovery + TCP transfers), concurrency primitives, and basic protocol design in a small, approachable codebase.

**This is a learning project — not production software.** Use it on trusted local networks only.

**What Orb does**
- Discovers nearby peers on the LAN (UDP-based discovery)
- Lets a peer advertise itself and choose to `Send` or `Receive`
- Performs a simple TCP handshake to request and transfer files
- Provides a tiny, terminal-based CLI for interactive send/receive

**Status**
- WIP / experimental: the code is intentionally simple and synchronous/blocking in places
- Not hardened for security, authentication, or large-scale reliability
- Uses ed25519 for node identity; the protocol and crypto have not been audited

**Why this project exists**
Orb is a focused exercise to learn how real-world file-sharing primitives fit together: discovery, handshakes, streaming, and state management with `Arc`/`RwLock`. It is a foundation for more ambitious experiments (referenced in the repo's history).

**Key files**
- [Cargo.toml](Cargo.toml) — project manifest and dependencies
- [src/main.rs](src/main.rs) — CLI entrypoint and main loop
- [src/discovery](src/discovery) — discovery, identity, and TCP handshake code
- [src/transfer.rs](src/transfer.rs) — file send/receive implementation

**Quickstart (build & run)**
Prerequisites: Rust toolchain (stable), `cargo` on your PATH.

Build a release binary:

```bash
cargo build --release
```

Run the program (two machines or two terminals on the same LAN):

```bash
# Start one instance and choose "Receive" to wait for files
target/release/orb

# Start another instance and choose "Send" to scan for receivers
target/release/orb
```

Usage summary (interactive):
- Choose `1` to Send: picks a receiver and asks for a file path to transfer
- Choose `2` to Receive: opens a TCP listener and waits for an incoming file
- Choose `3` to exit

**Security & limitations**
- Do not use Orb on untrusted networks — the protocol is minimal and intended for learning
- There is no hardened permission model or encrypted transport beyond any identity mechanism used
- Large file transfers and NAT traversal are not handled

**Contributing / Next steps**
- If you'd like improvements, consider: async IO (Tokio), authenticated/encrypted transfers, resumable transfers, progress reporting, and tests
- Open a PR or issue if you'd like this repo to evolve into a more robust tool

**License**
No `LICENSE` file is present in this repository. Add one if you want to publish the code with explicit terms.

---

If you want, I can: (a) make the README shorter, (b) add example screenshots/output, or (c) draft a `CONTRIBUTING.md` and `LICENSE` for the repo. Which would you like next?

Repository tour — what you'll find in this repo
- [Cargo.toml](Cargo.toml): Rust project manifest and dependency list.
- [README.md](README.md): (this) overview, quickstart, and architecture summary.
- [worker.d.ts](worker.d.ts): Type declarations (leftover/auxiliary file).
- [src/main.rs](src/main.rs): CLI entrypoint, main loop, and state machine.
- [src/transfer.rs](src/transfer.rs): File send/receive streaming logic (TCP read/write, basic framing).
- [src/discovery/mod.rs](src/discovery/mod.rs): Module glue for discovery.
- [src/discovery/discover.rs](src/discovery/discover.rs): UDP discovery beaconing and peer table maintenance.
- [src/discovery/identity.rs](src/discovery/identity.rs): Local identity generation and storage (ed25519 keypair persisted to config dir).
- [src/discovery/tcp_handshake.rs](src/discovery/tcp_handshake.rs): Outgoing TCP handshake and connect logic.
- [src/discovery/tcp_listener.rs](src/discovery/tcp_listener.rs): Incoming TCP listener and simple accept/reject prompt.

High-level system architecture

The following diagram shows the main runtime components and interactions. It's intentionally simple and focuses on separation of concerns: discovery, connection establishment, and file streaming.

```mermaid
flowchart LR
	CLI[CLI / main.rs] -->|reads/writes| State[State (Arc/RwLock)]
	CLI --> Discovery[Discovery Thread]
	Discovery -->|UDP broadcast| LAN[Local Network]
	LAN -->|beacons| Discovery
	Discovery --> PeerTable[(Peer Table)]
	CLI -->|Send request| TCPHandshake[tcp_handshake.rs]
	TCPHandshake -->|TCP connect| PeerListener[tcp_listener.rs]
	PeerListener -->|accepts| FileTransfer[transfer.rs]
	TCPHandshake --> FileTransfer
	Identity[identity.rs] --> CLI
	Identity --> TCPHandshake
	subgraph Storage
		IdentityFile[(~/.config/orb/identity.sk)]
	end
	IdentityFile --> Identity

	classDef infra fill:#f9f,stroke:#333,stroke-width:1px;
	class Discovery,TCPHandshake,PeerListener,FileTransfer infra;
```

Design highlights (recruiter-friendly points)
- Clean separation: discovery, handshake, transfer, and identity are split into small modules for testability and clarity.
- Practical primitives: uses UDP for LAN discovery, a dedicated TCP port for transfers, and simple framing for file metadata + streaming.
- Thoughtful persistence: the identity keypair is stored in the OS config directory (so node identity survives restarts).
- Concurrency model: minimal and explicit threading with `Arc` + `RwLock` for shared state — easy to understand and reason about.

If you'd like, I can now:
- Produce a condensed one-page README tailored for recruiters.
- Add a simple architecture diagram image and embed it in the README.
- Draft `CONTRIBUTING.md` and add an `MIT` or `Apache-2.0` `LICENSE`.

Tell me which of the three you'd like next and I'll implement it.
