.PHONY: sim flash

sim:
	cd sim && cargo r

flash:
	cd firmware && cargo run --release
