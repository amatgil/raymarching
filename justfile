
render:
        just clean
        cargo run --release
        just slideshow

convert:
     ./ppm_to_png.sh

slideshow:
        just convert
        ffmpeg -pattern_type glob -i "pngs/*.png" -c:v libx264 -r 24 output.mp4

clean:
        rm -f outputs/*
        rm -f pngs/*
        rm -f output.mp4

finalize name:
         mv output.mp4 finals/{{name}}.mp4