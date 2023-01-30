# Tor Analyzer

## Tor daemon step-up

This app should be run as a non-privileged user, so twick your tor configuration:

```conf
# Control port
ControlSocket /var/lib/tor/control.sock
ControlSocketsGroupWritable 1

# Cookie
CookieAuthentication 1
CookieAuthFileGroupReadable 1

# Data directory
DataDirectoryGroupReadable 1
```

This will allow users of the `tor` group to connect to the control socket.?

## Build

Just run the following command:

```bash
cargo build --release

```

## Enjoy

```bash
cargo run --release -- /var/lib/tor/control.sock
```

The only argument is the tor control socket (either IPv4/IPv6/Unix).
