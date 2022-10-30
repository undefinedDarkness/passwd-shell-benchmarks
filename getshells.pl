#!/usr/bin/env perl

use strict;

my %pwhash = ();

open(PW, "passwd") || die "trying";
while (chomp(my $line = <PW>)) {
    $line =~ /.*:(\S+)$/;
    $pwhash{$1}++;
}

foreach (sort keys (%pwhash)) {
	next if ( $pwhash{$_} eq "" );
        printf("%-18s:\t%d\n", $_, $pwhash{$_});
}

