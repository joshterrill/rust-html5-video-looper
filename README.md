# rust-html5-video-looper

a web app written with rust that continuously plays videos in a random order from a specified directory.

### Running in development

```
git clone https://github.com/joshterrill/rust-html5-video-looper
cd rust-html5-video-looper/
cargo run path ./public/videos maxplays 20
```

This will start a web server on http://localhost:8000 that will play your videos.

### Running binary

Example:

```
git clone https://github.com/joshterrill/rust-html5-video-looper
cd rust-html5-video-looper/
cargo build --release
cd target/release/
./rust-html5-video-looper path ./public/videos maxplays 20
```

This will also start a web server on http://localhost:8000 that will play your videos.