cargo build --release
a=target/release/qqmusic
strip $a
./$a
