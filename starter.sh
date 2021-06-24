sudo touch /var/run/article-storage.pid
sudo chmod 666 /var/run/article-storage.pid
./run.sh
nohup ./wathcer.sh >/dev/null 2>&1 &
