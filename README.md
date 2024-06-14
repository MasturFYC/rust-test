### Run and watch app
cargo watch -q -c -w src/ -x run


### references
https://codevoweb.com/building-a-rust-api-with-unit-testing-in-mind/

### PSQL restore from .tar file
pg_restore --verbose --clean --no-acl --no-owner -h localhost -U postgres -d mmdb file.tar