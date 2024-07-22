
render:
        rm -f outputs/*
        cargo run --release
        ffmpeg -pattern_type glob -i "*.ppm" -c:v libx264 -r 20 output.mkv