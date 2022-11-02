# Lekko Sidecar

## Development

Start with the [rust section of Getting Started](https://www.notion.so/lekko/Getting-Started-1296588c4ed44898820983c57b51f490#99f8d824ce504fed877a8f94b2a99860)

We have a rust-specific Makefile. Run `make` to run all `cargo` based steps.

### Regenerating protos

You need a few external dependencies to re-build the proto & grpc stubs.

```
cargo install protoc-gen-prost
cargo install protoc-gen-prost-tonic
buf generate --template buf.gen.yaml buf.build/lekkodev/cli
```
###

Running the dockerfile for now:
```
docker run -d -t --rm --name sidecar -p 50051:50051 -e LEKKO_BIND_ADDR='0.0.0.0:50051' -e RUST_BACKTRACE=1 docker.io/lekko/sidecar:latest
```
