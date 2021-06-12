# explorer-converter(WIP)

## meilisearch

```shell
sudo docker run -idt --rm \
    -p 7700:7700 \
    -v $(pwd)/data.ms:/data.ms \
    getmeili/meilisearch
```

## redis

```shell
sudo docker run --name redis -d -p 6379:6379 redis --requirepass "password"
```



## develop


### build converter docker image

```shell
sudo docker build -f Dockerfile . -t bingryan/converter:v1
```

#### default runtime for cn build

```shell
sudo docker build --build-arg DOMAIN=cn -f Dockerfile . -t bingryan/converter:v1
```

#### default runtime for cn build


```shell
sudo docker build --build-arg DOMAIN=cn --build-arg RUNTIME=node_template -f Dockerfile . -t bingryan/converter:v1
```

### run containers 

```shell
sudo docker-compose -f compose/docker-compose.yml up -d
```

### clear containers

```shell
sudo docker stop $(sudo docker ps -q) & sudo docker rm $(sudo docker ps -aq)
```