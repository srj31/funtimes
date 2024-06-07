#! /bin/bash

echo "Hello, World!"
count=10

# if [ $count -eq "$1" ]; then
# 	echo "Count is $1"
# else
# 	echo "Count is not 10 its $1"
# fi
#
while [ $count -lt 20 ]; do
	echo "Count is $count"
	((count++))
done

while read -r line; do
	echo "$line"
done <"${1:-/dev/stdin}"
