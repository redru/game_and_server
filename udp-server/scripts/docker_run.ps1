docker stop udp-server
docker run -it --rm --name udp-server -p 34254:34254/udp -d udp-server
