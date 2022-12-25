@include "../gendata/join.awk"
@include "../gendata/rust_arr_join.awk"
# NOTE: This script must remain compatible with BSD awk and GNU awk

BEGIN {
  FS=";"
}

# /^[0-9A-F]{4};/ should work but FreeBSD awk as of 12.1 doesn't support {}
# https://bugs.freebsd.org/bugzilla/show_bug.cgi?id=54410
/^[0-9A-F ]+;/ {
  rust_arr_split_chars($1, hex);
  joined=rust_arr_join(hex);

  printf("%12c%s => Some(Cow::from(\"%s\")),\n", " ", arr, $2, "")
}
