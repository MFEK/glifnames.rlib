# NOTE: This script must remain compatible with BSD awk and GNU awk

BEGIN {
  FS=";"
}

# /^[0-9A-F]{4};/ should work but FreeBSD awk as of 12.1 doesn't support {}
# https://bugs.freebsd.org/bugzilla/show_bug.cgi?id=54410
/^[0-9A-Za-z._-]+;[0-9A-F]+/ { printf("%12c0x%s => Some(Cow::from(\"%s\")), %"12+maxglyphname-length($1)-length($2)"s%s\n", " ", $2, $1, "", "// "$3) }
