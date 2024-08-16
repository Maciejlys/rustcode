#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: $0 <problem_number>"
    exit 1
fi

problem_number=$(printf "%03d" $1)
file_name="src/problem_$problem_number.rs"

if [ -e $file_name ]; then
    echo "Problem $problem_number already exists!"
    exit 1
fi

# Copy the template and rename the module inside it
cp template/problem_template.rs $file_name
sed -i "" "s/template/problem_$problem_number/" $file_name

# Append the new module to lib.rs
echo "pub mod problem_$problem_number;" >> src/lib.rs

echo "Problem $problem_number created and linked in lib.rs"
