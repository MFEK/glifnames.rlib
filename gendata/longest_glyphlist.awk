BEGIN {
  FS=";"
}

/^[0-9A-Fa-z._]+;/ { maxglyphname=length($1) > maxglyphname ? length($1) : maxglyphname; }

END {
  print maxglyphname
}
