#!/usr/bin/perl

$count=0;
while(<>) {
  /(\d+)-(\d+) (\w): (.*)/;
  $count++ if $4 =~ /^[^${3}]*(${3}[^${3}]*){$1,$2}[^${3}]*$/;
}
print "$count\n";


# One liner version : 
# 1-
# perl -ne '{/(\d+)-(\d+) (\w): (.*)/;$c++ if $4 =~ /^[^$3]*($3[^$3]*){$1,$2}[^$3]*$/}END{print "$c\n";}' < input.txt
# 2-
# perl -ne '{/(\d+)-(\d+) (\w): (.*)/;$a=$1-1;$b=$2-$1-1;$c++ if $4 =~ /^.{$a}$3.{$b}[^$3]|^.{$a}[^$3].{$b}$3/}END{print "$c\n";}' < input.txt



