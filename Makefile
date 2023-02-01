# binary name
BINARY=rhc

# default target
all: build

# build target
build:
	cargo build --release

# install target
install: build
	uname_S = $(shell uname -s)
	ifeq ($(uname_S), Linux)
		sudo cp target/release/$(BINARY) /usr/local/bin/$(BINARY)
	endif
	ifeq ($(uname_S), Darwin)
		sudo cp target/release/$(BINARY) /usr/local/bin/$(BINARY)
	endif

# clean target
clean:
	cargo clean

