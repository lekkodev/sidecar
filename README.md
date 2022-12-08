# Lekko Sidecar

## Development

Start with the [rust section of Getting Started](https://www.notion.so/lekko/Getting-Started-1296588c4ed44898820983c57b51f490#99f8d824ce504fed877a8f94b2a99860)

We have a rust-specific Makefile. Run `make` to run all `cargo` based steps.

### Regenerating protos

You need a few external dependencies to re-build the proto & grpc stubs.

```
cargo install protoc-gen-prost
cargo install protoc-gen-prost-crate
cargo install protoc-gen-tonic
buf generate --template buf.gen.yaml buf.build/lekkodev/cli
```

### Building docker files

Run:
```
make dockerbuild
```
Or if you want an image for amd64 run:
```
make dockerbuild amd64
```

## Deploying

Running the dockerfile for now:
```
docker run -d -t --rm --name sidecar -p 50051:50051 -e RUST_BACKTRACE=1 docker.io/lekko/sidecar:latest
```

### Command-line args

The binary runs with the following args:
```
Lekko sidecar that provides the host application with config updates from Lekko and performs local evaluation

Usage: sidecar [OPTIONS]

Options:
  -l, --lekko-addr <LEKKO_ADDR>
          Address to communicate with lekko backend.. [default: https://grpc.lekko.dev]
  -b, --bind-addr <BIND_ADDR>
          Address to communicate with lekko backend.. [default: 0.0.0.0:50051]
  -p, --proxy-mode
          Enabling proxy mode will run server-side evaluation instead of local evaluation
  -f, --fallback-repo-path <FALLBACK_REPO_PATH>
          Absolute path to the fallback config repository on local disk. If not provided, there will be no fallback behavior
  -h, --help
          Print help information
  -V, --version
          Print version information
```
