#!/bin/bash

while IFS= read -r line
do
   echo "Line: $line"
done >> input.txt
