install:
	@echo "running on:"
	@uname -o
	@echo "compiling and installing ferrissay v0.1.1-beta"
	@echo "please wait..."
	@cargo build --release --quiet --target-dir build
	@sudo install -m 755 build/release/ferrissay /usr/local/bin/ferrissay
	@echo "ferrissay installed at /usr/local/bin/ferrissay"
	@echo "see https://github.com/ghlilith1k/ferrissay for details"
	@echo "usage: ferrissay [-u] [message]"