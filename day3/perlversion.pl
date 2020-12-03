# oneliner for the first step :
#  w=$(head -1 input.txt | wc -c); tr '\n' ' ' < input.txt | perl -pe 's/(?:(.)(?=[^ ]{0,2} ).{3}|(.).{'$((2+w))'}|(.).*)/$1$2$3/g'  | tr -d '.' | wc -c
#
