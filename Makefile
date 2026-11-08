install:
	@echo "ferrissay v0.1.1-beta, (c) 2026 ghlilith1k"
	@echo "see https://github.com/ghlilith1k/ferrissay for details"
	@echo "running on:"
	@uname -o
	@echo "compiling and installing ferrissay v0.1.1-beta"
	@echo "please wait..."
	@cargo build --release --quiet
	@sudo install -m 755 target/release/ferrissay /usr/local/bin/ferrissay
	@echo "ferrissay installed at /usr/local/bin/ferrissay"
	@echo " usage: ferrissay [-u] [message]"