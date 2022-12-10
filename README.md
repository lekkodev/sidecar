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
  -l, --lekko-addr <LEKKO_ADDR>        Address to communicate with lekko backend.. [default: https://grpc.lekko.dev]
  -b, --bind-addr <BIND_ADDR>          Address to communicate with lekko backend.. [default: 0.0.0.0:50051]
  -p, --proxy-mode                     Enabling proxy mode will run server-side evaluation instead of local evaluation
  -r, --repo-path <REPO_PATH>          Absolute path to the directory on disk that contains the .git folder. Provide this flag to turn on bootstrap behavior
  -c, --contents-path <CONTENTS_PATH>  Path to the directory on disk that contains the repo contents (lekko.root.yaml). If none, it is assumed that the contents are in repo_path, which is the case for most local clones of a git repo. git-sync is the exception, as it houses contents in a separate symlinked directory
  -h, --help                           Print help information
  -V, --version                        Print version information
```
