.PHONY: sim flash

sim:
	cargo run -p lebron-simulator

flash:
	cd firmware && cargo run --release
