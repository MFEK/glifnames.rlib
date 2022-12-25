# NOTE: This Makefile must remain compatible with BSD make and GNU make
# The file `GNUmakefile` makes testing this easier on GNU/Linux.

SHELL=/bin/sh

.SUFFIXES:.rs .txt

.EXEC:
all:
	$(MAKE) $(MFLAGS) -C src aglfn.rs
	$(MAKE) $(MFLAGS) -C src glyphlist.rs
	mv src/glyphlist.rs src/legacy_agl.rs

.rs: .txt
