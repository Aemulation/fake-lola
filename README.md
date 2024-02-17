## Setup

1. Create a virtual webcam device

```bash
sudo modprobe v4l2loopback video_nr=42
```

2. Stream a picture to it using ffmpeg

```bash
ffmpeg -loop 1 -i camera_data/rgb_1.jpg -s 640x480 -f v4l2 -vcodec rawvideo -pix_fmt yuyv422 /dev/video42
```

3. In `crates/heimdall/src/camera`, change the video device variables `CAMERA_BOTTOM` and `CAMERA_TOP` to `/dev/video42`

4. To get rerun working, in `/yggdrasil/src/debug/mod.rs`, change

```rust
let server_address = Ipv4Addr::new(10, 0, 8, robot_info.robot_id as u8);
```

to

```rust
let server_address = Ipv4Addr::LOCALHOST;
```

5. Disable the rotation for the top camera. Remove the following lines in `yggdrasil/crates/heimdall/src/camera.rs`

```rust
camera_device.horizontal_flip()?;
camera_device.vertical_flip()?;
```


## Run `fake-lola`

```bash
cargo run -r
```

## Run `yggdrasil`
Without `rerun` support
```bash
cd deploy
ROBOT_ID=20 ROBOT_NAME=sam cargo run -r
```
Or with `rerun` support
```bash
cd deploy
ROBOT_ID=20 ROBOT_NAME=sam cargo run -r --features rerun
```
