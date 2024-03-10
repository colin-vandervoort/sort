#!/bin/bash

set -eu

script_path="$(cd -- "$(dirname "$0")" &> /dev/null && pwd -P )"
project_path="$(cd -- "${script_path}/.." &> /dev/null  && pwd -P)"
pokemon_data_path="${project_path}/tests/data/Pokemon.csv"

usage() {
    echo "Usage: $0 <OPTIONS>"
    echo "    OPTIONS:"
    echo "        -c <NUM>       column index"
    echo "        -s <SEPSTRING> output separator"
    echo "        -m             filter out 'Mega' variants"
    echo "        -k             Kanto region only"
    exit 1
}

col_index=1
sep='\n'
remove_megas=1
kanto_only=1

while getopts c:s:mk name
do
    case $name in
        c)  col_index="$OPTARG"
            ;;
        s)  sep="$OPTARG"
            ;;
        m)  remove_megas=0
            ;;
        k)  kanto_only=0
            ;;
        *)  usage
            ;;
    esac
done

pre_processed=$(cat "$pokemon_data_path")

if [ "$remove_megas" ]; then
    pre_processed=$(echo "$pre_processed" | grep --invert-match '.*Mega')
fi

if [ "$kanto_only" ]; then
    pre_processed=$(echo "$pre_processed" | awk -v FS=',' '$1 >= 1 && $1 <= 151')
fi

echo "$pre_processed" | awk -v FS=',' -v ORS="$sep" -v column=$((col_index+1)) 'NR > 1 { print $column }'
