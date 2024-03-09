#!/bin/bash

set -e
set -o pipefail

CAMERA_DEVICE_PATH="/dev/video42"

camera_device_already_exists() {
	for device_path in /dev/*; do
		if [ "$device_path" == "${CAMERA_DEVICE_PATH}" ]; then
			return 0
		fi
	done

	return 1
}

if [[ -z $1 ]]; then
	echo "Usage: run.sh image.jpeg"
	exit 0
fi

if ! camera_device_already_exists; then
	sudo modprobe v4l2loopback video_nr=42 max_buffers=3
fi

printf "running..."

if file -ib "$1" | grep -q image; then
	parallel ::: "cargo run -r" "ffmpeg -re -loop 1 -i \"$1\" -filter:v fps=30 -s 640x480 -f v4l2 -vcodec rawvideo -pix_fmt yuyv422 ${CAMERA_DEVICE_PATH}"
elif file -ib "$1" | grep -q video; then
	parallel ::: "cargo run -r" "ffmpeg -stream_loop -1 -re -i \"$1\" -s 640x480 -vcodec rawvideo -pix_fmt yuyv422 -f v4l2 ${CAMERA_DEVICE_PATH}"
else
	echo "Invalid input file"
	exit 0
fi
