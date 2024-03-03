## Setup

1. Create a virtual webcam device

```bash
sudo modprobe v4l2loopback video_nr=42 max_buffers=3
```

2. Stream a picture to it using ffmpeg

```bash
ffmpeg -re -loop 1 -i camera_data/rgb_1.jpg -filter:v fps=30 -s 640x480 -f v4l2 -vcodec rawvideo -pix_fmt yuyv422 /dev/video42
```

## Run `fake-lola`

```bash
cargo run -r
```

## Run `yggdrasil`
Without `rerun` support
```bash
cd deploy
ROBOT_ID=0 ROBOT_NAME=local cargo run -r --features local
```
Or with `rerun` support
```bash
cd deploy
ROBOT_ID=0 ROBOT_NAME=local cargo run -r --features local,rerun
```
