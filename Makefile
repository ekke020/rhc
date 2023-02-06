TARGET = rhc
BUILD_TYPE = release

release: BUILD_TYPE = release
release: build

debug: BUILD_TYPE = debug
debug: build

# Define the build command
BUILD_CMD = cargo build --$(BUILD_TYPE) --future-incompat-report 

# Define the install command
INSTALL_CMD = cargo install --force --path .

# Define the clean command
CLEAN_CMD = cargo clean -v

# Define the uninstall command
UNINSTALL_CMD = cargo uninstall $(TARGET)

build:
	$(BUILD_CMD)

install:
	$(INSTALL_CMD)

uninstall:
	$(UNINSTALL_CMD)

clean:
	$(CLEAN_CMD)

.PHONY: build install uninstall clean debug release

default: BUILD_TYPE = release
default: build