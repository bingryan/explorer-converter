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

# error

```shell
cargo update -p jsonrpsee-utils --precise 0.2.0-alpha.3
```



## develop


### build converter docker image

```shell
sudo docker build -f Dockerfile .
```

#### cn
```shell
docker build -f Dockerfile --build-arg DOMAIN=cn .

```

### run containers 

```shell
sudo docker-compose -f compose/docker-compose.yml up -d
```