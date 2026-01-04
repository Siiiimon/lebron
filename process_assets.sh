#!/usr/bin/env bash

for input_file in assets/raw/*.bmp; do
    [ -e "$input_file" ] || continue

    filename=$(basename "$input_file")

    output_file="assets/$filename"

    echo "Processing $filename..."

    ffmpeg -i "$input_file" -pix_fmt rgb24 -y "$output_file" -v error -stats
done

echo "Done processing bmps"
