#!/bin/sh

TYPEDB=${TYPEDB-'typedb'}

grep -Po '::\K.*$' $TYPEDB |
while read -r pattern
do
    if echo "$1" |
	    sed -e 's/^[[:space:]]*//' -e 's/[[:space:]]*$//' |
	    grep -xq "${pattern}";
    then
	grep -A 2 -Fx "::${pattern}" $TYPEDB |
        grep -m 1 -Po "$2: \K.*$" &&
        echo "^-- in pattern ${pattern}" 1>&2 &&
        exit
    else
	if [ $? -eq 2 ]; then
	   echo "^-- in pattern ${pattern}" 1>&2
	fi
    fi
done

