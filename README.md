# Prerequisites

- `v4l2loopback`
- `ffmpeg`
- `cargo`
- `parallel` for executing shell scripts in parallel.

## Run `fake-lola`

```bash
./run.sh camera_data/rgb_1.jpeg
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
