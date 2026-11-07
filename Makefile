install:
	@echo "running on:"
	@uname -o
	@cargo build --release --target-dir build
	@sudo install -m 755 build/release/ferrissay /usr/local/bin/ferrissay
	@echo "ferrissay installed at /usr/local/bin/ferrissay"
	@echo "usage: ferrissay [something...]"