### Run and watch app
cargo watch -q -c -w src/ -x run


### references
https://codevoweb.com/building-a-rust-api-with-unit-testing-in-mind/

### PSQL restore from .tar file
pg_restore --verbose --clean --no-acl --no-owner -h localhost -U postgres -d mmdb file.tar

### LOGIN
curl -X POST localhost:8000/api/auth/login -H 'content-type: application/json' -d '{"email":"mastur.st12@gmail.com", "password":"xxx"}'

### Write token to file
echo "token" > token.txt
