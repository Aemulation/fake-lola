#!/bin/bash

if [ -z $1 ]; then
	echo "Usage: run.sh image.jpeg"
	exit 0
fi

if [ ! -d /dev/video42 ]; then
	sudo modprobe v4l2loopback video_nr=42 max_buffers=3
fi

printf "running..."

parallel ::: "cargo run -r" "ffmpeg -re -loop 1 -i ${1} -filter:v fps=30 -s 640x480 -f v4l2 -vcodec rawvideo -pix_fmt yuyv422 /dev/video42"
