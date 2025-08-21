.PHONY: all build install clean

BINARY_NAME = rtracer
TARGET_PATH = target/debug/$(BINARY_NAME)
INSTALL_PATH = /usr/bin/$(BINARY_NAME)

all: build

build:
	cargo build

install: build
	cp $(TARGET_PATH) $(INSTALL_PATH)
	chmod +x $(INSTALL_PATH)

clean:
	cargo clean
