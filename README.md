# How to start

Start a postgres database
```
docker run -d -p 5432:5432 -e POSTGRES_PASSWORD=password -e POSTGRES_USER=username postgres
```

Init database
```
cargo database reset
```

Start server
```
cargo run
```
