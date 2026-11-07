install:
	@cargo build --release
	@sudo cp target/release/ferrissay /usr/local/bin/ferrissay
	@rm -rf target
	@echo "ferrissay installed at /usr/local/bin/ferrissay"
	@echo "usage: ferrissay [something...]"