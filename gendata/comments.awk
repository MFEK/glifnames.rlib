BEGIN {
  FS="# "
  count=0
  count2=0
}

/---------/ { count++ }
count==1 && NR==1 {print "//! ```plain"}
count==2 {count++; print "//! ```"}
/^#/ { if (count>1) count2++; print (count == 1 ? "//! " : "/// "((count2==1) ? "```plain\n"(count == 1 ? "//! " : "/// ") : ($2 == "END" ? "```" : ""))) ($2 != "END" ? $2 : "") }
