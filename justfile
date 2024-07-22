
render:
        rm -f outputs/*
        rm output.mp4
        cargo run --release
        ffmpeg -pattern_type glob -i "outputs/*.ppm" -c:v libx264 -r 20 output.mp4