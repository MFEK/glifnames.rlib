# NOTE: This Makefile must remain compatible with BSD make and GNU make
# The file `GNUmakefile` makes testing this easier on GNU/Linux.

SHELL=/bin/sh

.SUFFIXES:.rs .txt

.EXEC:
all:
	$(MAKE) $(MFLAGS) -C src aglfn.rs
	$(MAKE) $(MFLAGS) -C src glyphlist.rs
	mv src/glyphlist.rs src/legacy_agl.rs
	$(MAKE) $(MFLAGS) -C src zapfdingbats.rs

.rs: .txt

.PHONY:
clean:
	/bin/bash -c 'for f in `ls src/{aglfn,glyphlist,zapfdingbats}.{,rs,txt}`; do rm "$$f" || true; done'
