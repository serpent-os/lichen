# lichen_ipc

This crate contains the `lichen_ipc` executable, a helper backend for the lichen
installer that communicates using the [varlink](https://varlink.org) protocol over
a local UNIX socket.

It is intended to be launched by a compatible frontend via some escalation system, such
as `pkexec` or `sudo`, ensuring that the frontend can communicate with the backend without
running as root itself.

It is in a very early stage, but eventually will be the entire backend of our installer.

## Testing

First, launch the backend:

```sh
pkexec ./target/debug/lichen_ipc --varlink=unix:@testinglichen
```

Then, in another terminal, run the [varlink cli](https://crates.io/crates/varlink-cli)

### Disk enumeration

```sh
varlink call unix:@testinglichen/com.serpentos.lichen.disks.GetDisks
```
```json
{
  "disks": [
    {
      "block_size": 512,
      "kind": "ssd",
      "model": "WD_BLACK SN770 1TB",
      "path": "/dev/nvme0n1",
      "size": 1953525168,
      "vendor": null
    }
  ]
}
```

### Partition enumeration

```sh
varlink call unix:@testinglichen/com.serpentos.lichen.disks.GetPartitions '{"disk": "/dev/sda"}'
```

```json
{
  "partitions": [
    {
      "kind": "esp",
      "path": "/dev/nvme0n1p1",
      "size": 134217728,
      "superblock_kind": "unknown",
      "uuid": "3a1338aa-d98b-45ef-b72b-62f9752ef2d2"
    },
    {
      "kind": "xbootldr",
      "path": "/dev/nvme0n1p2",
      "size": 1073741824,
      "superblock_kind": "unknown",
      "uuid": "915127ba-acd8-45db-a865-a9d8329d6f26"
    },
    {
      "kind": "regular",
      "path": "/dev/nvme0n1p3",
      "size": 4294967296,
      "superblock_kind": "unknown",
      "uuid": "2b89b961-4145-498f-8fb7-7b3178905c75"
    },
    {
      "kind": "regular",
      "path": "/dev/nvme0n1p4",
      "size": 994700165120,
      "superblock_kind": "ext4",
      "uuid": "e1ee771f-0c94-45fe-b544-2d22e50784e0"
    }
  ]
}
```
