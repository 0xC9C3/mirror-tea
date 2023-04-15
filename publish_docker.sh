VERSION=0.1.0

docker build -t 0xc9c3/mirror-tea:$VERSION --no-cache .
docker tag 0xc9c3/mirror-tea:$VERSION 0xc9c3/mirror-tea:latest
docker push 0xc9c3/mirror-tea:$VERSION
docker push 0xc9c3/mirror-tea:latest