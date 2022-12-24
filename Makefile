# NOTE: This Makefile must remain compatible with BSD make and GNU make
# The file `GNUmakefile` makes testing this easier on GNU/Linux.

SHELL=/bin/sh

../src/aglfn.rs: agl-aglfn/aglfn.txt gendata/aglfn.awk
	$(DOBEFORE) && (\
	cd gendata &&\
	LONGEST=$$(sort ../agl-aglfn/aglfn.txt | $$AWK -f longest.awk) &&\
	sort ../agl-aglfn/aglfn.txt | $$AWK -f aglfn.awk -v maxglyphname=$$LONGEST > $@ \
	)

.include <mk/before.mk>
