#!/usr/bin/env bash

echo "Cleaning previous assets..."
rm -f assets/*.tga assets/*.raw

for input_file in assets/raw/*.bmp; do
    [ -e "$input_file" ] || continue

    filename=$(basename "$input_file" .bmp)
    output_file="assets/${filename}.tga"

    echo "Processing $filename..."

    magick "$input_file" -type TrueColor -depth 5 -compress RLE "$output_file"
done

for input_file in assets/raw/*.wav; do
    [ -e "$input_file" ] || continue

    filename=$(basename "$input_file" .wav)
    output_file="assets/${filename}.raw"

    echo "Processing: $filename..."

    ffmpeg -i "$input_file" -ar 22050 -ac 1 -f s16le -y "$output_file" -v error -stats
done

echo "Done processing assets!"

du -hc assets/*.tga assets/*.raw
