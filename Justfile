start-db:
    docker run -it \
        -p 5432:5432 \
        -e POSTGRES_USER=poker \
        -e POSTGRES_PASSWORD=password \
        -v ./migrations:/docker-entrypoint-initdb.d \
        postgres
run:
    cargo run --release

test: 
    cargo test --release
    