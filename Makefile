default: help

build: ## Build the release version
	@cargo build --release

build-windows: ## Build the release version for windows
	@cargo build --release --target=x86_64-pc-windows-gnu

build-all: ## Build the release version for all platforms
	@cargo build --release --target=x86_64-pc-windows-gnu
	@cargo build --release --target=x86_64-apple-darwin
	@cargo build --release --target=x86_64-unknown-linux-gnu

help: ## Display this help screen
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m\033[0m\n"} /^[$$()% a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)
