git pull
cargo build --release
nohup ./target/release/article-storage &
echo $! > /var/run/article-storage.pid
