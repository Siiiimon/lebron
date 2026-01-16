#!/usr/bin/env bash

echo "Cleaning previous assets..."
rm -f assets/*.tga

for input_file in assets/raw/*.bmp; do
    [ -e "$input_file" ] || continue

    filename=$(basename "$input_file" .bmp)
    output_file="assets/${filename}.tga"

    echo "Processing $filename..."

    magick "$input_file" -type TrueColor -depth 5 -compress RLE "$output_file"
done

echo "Done processing assets!"

du -hc assets/*.tga assets/*.raw
