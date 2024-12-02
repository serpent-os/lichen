# lichen_rpc

This crate contains the `lichen_rpc` executable, a helper backend for the lichen
installer that communicates using the [varlink](https://varlink.org) protocol over
a local UNIX socket.

It is intended to be launched by a compatible frontend via some escalation system, such
as `pkexec` or `sudo`, ensuring that the frontend can communicate with the backend without
running as root itself.

It is in a very early stage, but eventually will be the entire backend of our installer.

## Testing

First, launch the backend:

```sh
pkexec ./target/debug/lichen_rpc --varlink=unix:@testinglichen
```

Then, in another terminal, run the [varlink cli](https://crates.io/crates/varlink-cli)

```sh
varlink call unix:@testinglichen/com.serpentos.lichen.disks.GetDisks
varlink call unix:@testinglichen/com.serpentos.lichen.disks.GetPartitions '{"disk": "/dev/sda"}'
```
