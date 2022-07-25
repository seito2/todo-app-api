# Get started

## For Podman user
```bash
./run_db.sh # run db
./api_binary # run api
```

## For Docker user
```bash
docker run --rm -d \
    -p 15432:5432 \
    -v postgres-tmp:/var/lib/postgresql/data \
    -e POSTGRES_HOST_AUTH_METHOD=trust \
    -e POSTGRES_PASSWORD=password \
    -e POSTGRES_USER=admin \
    -e POSTGRES_PASSWORD=admin \
    -v $(pwd)/init.d:/docker-entrypoint-initdb.d \
    postgres:12-alpine

./api_binary # run api
```