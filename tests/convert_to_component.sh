#!/bin/sh

INPUT_FOLDER=$1
OUTPUT_FOLDER=$2
ADAPTER=$3

if [ $# -lt 3 ]; then
  echo "Usage: ./convert_to_component.sh [input directory] [output directory] [adapter]"
  exit 1
fi

if [ -z "$INPUT_FOLDER" ]; then
  echo "Error: Invalid Input Folder"
  exit 1
fi

if [ -z "$OUTPUT_FOLDER" ]; then
  echo "Error: Invalid Output Folder"
  exit 1
fi

if [ -z "$ADAPTER" ]; then
  echo "Error: Adapter"
  exit 1
fi

no_errors=0

for file in "$INPUT_FOLDER"/*; do
  filename=$(basename -- "$file")
  extension="${filename##*.}"
  filename="${filename%.*}"
  if [ "$extension" = "wat" ]; then
    wat2wasm "$file" -o "$INPUT_FOLDER"/"$filename.wasm"
    if [ $? -ne 0 ]; then
      echo "wat2wasm failed on $file"
      no_errors=1
      continue
    fi

    wasm-tools component new -t --adapt "$ADAPTER" "$INPUT_FOLDER"/"$filename.wasm" -o "$OUTPUT_FOLDER"/"$filename.wat"
    if [ $? -ne 0 ]; then
      echo "wasm-tools component new failed on $file"
      no_errors=1
      continue
    fi
  fi
done

if [ $no_errors -eq 0 ]; then
  echo "All files converted successfully!"
fi