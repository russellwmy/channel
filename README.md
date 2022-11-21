# Channel

A web3 audio chat app for gamers & metaverse adventurers

## Folders

- `app` contains web frontend codebase
- `contracts` contains all smart contracts
- `docs` contains related documents, e.g. architecture design
- `template` is the template project for a new project
- `webrtc` contains webrtc client for audio chat
- `signalling-server` contains signalling server logic

## Techstacks

### Build tools

- [Rust](https://www.rust-lang.org/)
- [Cargo make](https://github.com/sagiegurari/cargo-make)
- [Trunk](https://github.com/thedodd/trunk)

### Frontend

- [Dioxus](https://dioxuslabs.com/)
- [Tailwindcss](https://tailwindcss.com/)
- [DaisyUI](https://daisyui.com/)
- [WebRTC](https://github.com/webrtc-rs/webrtc)

### Sigaling

- [WebRTC](https://github.com/webrtc-rs/webrtc)

### NEAR client SDK

- [Web3 Anywhere](https://github.com/russellwmy/web3-anywhere)

## Get Start
- in `signalling-server` folder run command: `cargo make serve` for signalling
- in `app` folder run command: `cargo make dev` for web frontend
- go to `http://localhost:8080`
