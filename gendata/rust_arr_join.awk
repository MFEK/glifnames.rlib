function rust_arr_split_chars(s, hex) {
  split(s, hex, " ");
  for (i = 1; i <= length(hex); i++) {
    hex[i] = ("0x"hex[i]"u32"); 
  }
}

function rust_arr_join(a) {
  joined=join(a, 1, length(a), ", ");
  arr="&["joined"]";
  return arr;
}
