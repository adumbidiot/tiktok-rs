# tiktok-rs
A scraping-based Rust library for tiktok.

## CLI
This repository contains a small CLI to download tiktok videos.

### Installation
```bash
cargo install --git https://github.com/adumbidiot/tiktok-rs
```

### Usage
A video can be downloaded with the following command:
```bash
tiktok-cli <video url>
```

The video is named `<video id>.mp4`. 
If the file already exists, it is not downloaded.