ADDON=thing
LIBNAME=libthing.so

default: debug
debug:
	cargo build
	ln -sf ../../../../../target/debug/$(LIBNAME) project/addons/$(ADDON)/bin/linux/$(LIBNAME)

r: release
release:
	cargo build --release
	ln -sf ../../../../../target/release/$(LIBNAME) project/addons/$(ADDON)/bin/linux/$(LIBNAME)

