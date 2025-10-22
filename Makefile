default: build

build:
	stellar contract build
	@ls -l target/wasm32v1-none/release/*.wasm

deploy:
	stellar contract deploy \
	--wasm target/wasm32v1-none/release/hello_world.wasm \
	--source alice \
	--network testnet
