#!/usr/bin/perl

$count=0;
while(<>) {
  /(\d+)-(\d+) (\w): (.*)/;
  $count++ if $4 =~ /^[^${3}]*(${3}[^${3}]*){$1,$2}[^${3}]*$/;
}
print "$count\n";


# One liner version : 
# perl -ne '{/(\d+)-(\d+) (\w): (.*)/;$count++ if $4 =~ /^[^${3}]*(${3}[^${3}]*){$1,$2}[^${3}]*$/}END{print "$count\n";}' < input.txt

