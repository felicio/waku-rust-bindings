# Waku Rust bindings

Rust layer on top of [`go-waku`](https://github.com/status-im/go-waku) [c ffi bindings](https://github.com/status-im/go-waku/blob/v0.2.2/library/README.md).


## About [Waku](https://waku.org/)

Waku is the communication layer for Web3. Decentralized communication that scales.

Private. Secure. Runs anywhere.

### What is Waku?

Waku is a suite of privacy-preserving, peer-to-peer messaging protocols.

Waku removes centralized third parties from messaging, enabling private, secure, censorship-free communication with no single point of failure.

Waku provides privacy-preserving capabilities, such as sender anonymity,metadata protection and unlinkability to personally identifiable information.

Waku is designed for generalized messaging, enabling human-to-human, machine-to-machine or hybrid communication.

Waku runs everywhere: desktop, server, including resource-restricted devices, such as mobile devices and browsers.
How does it work?

The first version of Waku had its origins in the Whisper protocol, with optimizations for scalability and usability. Waku v2 is a complete rewrite. Its relay protocol implements pub/sub over libp2p, and also introduces additional capabilities:

1. Retrieving historical messages for mostly-offline devices. 
2. Adaptive nodes, allowing for heterogeneous nodes to contribute. 
3. Bandwidth preservation for light nodes.

This makes it ideal for running a p2p protocol on mobile, or in other similarly resource-restricted environments.



Read the [Waku docs](https://docs.wakuconnect.dev/)