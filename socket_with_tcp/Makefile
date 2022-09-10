ifeq ($(shell uname),Darwin)
    EXT := dylib
else
    EXT := so
endif

all: libsock_client.$(EXT)
	g++ cpp/main.cpp -L "./target/x86_64-apple-darwin/release" -lsock_client -o run -arch x86_64

deploy:
	cp ./target/x86_64-apple-darwin/release/libsock_client.$(EXT) /usr/local/lib

libsock_client.$(EXT): sock_client/src/lib.rs Cargo.toml
	cargo build --release --target=x86_64-apple-darwin

clean:
	rm -rf target
	rm -rf run