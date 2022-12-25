.SUFFIXES:
.SUFFIXES: .rs .txt

.include "../Makefile"

.ONESHELL:
aglfn.rs glyphlist.rs:
	$(DOBEFORE) && (\
	input=$$(echo $@ | sed -e 's@^@agl-aglfn/@; s/\.rs$$/.txt/;') &&\
	filename=$$(echo $$input | sed -e 's@agl-aglfn/@@; s@\.txt@@;') &&\
	echo $$filename &&\
	cat ../gendata/header_common.rs.template > ${.TARGET} &&\
	cat ../gendata/$${filename}_abovecomment.rs.template >> ${.TARGET} &&\
	awk -f ../gendata/comments.awk -v midcomment=../gendata/$${filename}_midcomment.rs.template < ../$$input >> ${.TARGET} &&\
	cat ../gendata/$${filename}_header.rs.template >> ${.TARGET} &&\
	([ -f ../gendata/$${filename}_fn1def.rs.template ] && cat ../gendata/$${filename}_fn1def.rs.template >> ${.TARGET} || true) &&\
	LONGEST=$$(sort ../$$input | $$AWK -f ../gendata/longest_$${filename}.awk) &&\
	sort ../$$input | $$AWK -f ../gendata/$${filename}.awk -v maxglyphname=$$LONGEST >> $${filename}.rs &&\
	([ -f ../gendata/$${filename}_fn1foot.rs.template ] && cat ../gendata/$${filename}_fn1foot.rs.template >> ${.TARGET} || true) &&\
	([ -f ../gendata/$${filename}_fn2def.rs.template ] && cat ../gendata/$${filename}_fn2def.rs.template >> ${.TARGET} || true) &&\
	([ -f ../gendata/$${filename}_fn2.awk ] && sort ../$$input | gawk -f ../gendata/$${filename}_fn2.awk -v maxglyphname=$$LONGEST >> $${filename}.rs || true) &&\
	([ -f ../gendata/$${filename}_fn2foot.rs.template ] && cat ../gendata/$${filename}_fn2foot.rs.template >> ${.TARGET} || true) &&\
	cat ../gendata/$${filename}_footer.rs.template >> $${filename}.rs\
	)

.include "../mk/before.mk"
