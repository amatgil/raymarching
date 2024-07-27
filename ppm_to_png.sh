#!/bin/sh

for i in outputs/*.ppm;
  do name=`echo "$i" | xargs -I{} basename {} | cut -d'.' -f1`
  echo "Turning $name.ppm into pngs/$name.png"
  magick "$i" "pngs/$name.png"
done
