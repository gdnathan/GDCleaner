BINARY = gdcleaner
INSTALL_DIR ?= $(HOME)/.local/bin
CONFIG_DIR ?= $(or $(XDG_CONFIG_HOME),$(HOME)/.config)/gdcleaner
 
.PHONY: all build install uninstall clean
 
all: build
 
build:
	cargo build --release
 
install: build
	@mkdir -p $(INSTALL_DIR)
	cp target/release/$(BINARY) $(INSTALL_DIR)/$(BINARY)
	chmod +x $(INSTALL_DIR)/$(BINARY)
	@mkdir -p $(CONFIG_DIR)
	@if [ ! -f $(CONFIG_DIR)/config.toml ]; then \
		cp config.toml $(CONFIG_DIR)/config.toml; \
		echo "Config installed to $(CONFIG_DIR)/config.toml"; \
	else \
		echo "Config already exists, skipping."; \
	fi
	@echo "Installed $(BINARY) to $(INSTALL_DIR)"
 
uninstall:
	rm -f $(INSTALL_DIR)/$(BINARY)
	@echo "Removed $(INSTALL_DIR)/$(BINARY)"
	@echo "Config at $(CONFIG_DIR) was left untouched."
 
clean:
	cargo clean
 
