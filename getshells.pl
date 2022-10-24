#!/usr/bin/perl

open(PW, "passwd") || die "trying";

while (chomp($line = <PW>)) {
  ($foo,$bar,$baz,$quux,$goo,$googoo,$shell) = split /:/, $line;
#  printf("%s\n", $shell);
  $pwhash{$shell}++;
  
}

foreach (sort keys (%pwhash)) {
	next if ( $pwhash{$_} eq "" );
        printf("%-18s:\t%d\n", $_, $pwhash{$_});
}

