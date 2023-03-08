#!/usr/bin/env perl

use strict;

my %pwhash;
my $filename = "passwd";

# Open the password file for reading
open(my $fh, '<', $filename) or die "Could not open file '$filename' $!";

# Process each line of the file
while (my $line = <$fh>) {
    chomp($line);
    # Split the line into fields using ':' as the delimiter
    my @fields = split(':', $line);
    # Extract the last field, which contains the login shell
    my $shell = $fields[-1];
    # Increment the count of accounts using this shell
    $pwhash{$shell}++;
}

# Print the output in three columns with vertical alignment
foreach my $shell (keys %pwhash) {
    # Skip empty shells
    next unless $shell;
    # Print the shell and the count of accounts using it
    printf("%-18s:\t%d\n", $shell, $pwhash{$shell});
}

# Close the file handle
close($fh);
