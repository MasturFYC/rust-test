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

### setup for running rust-analyzer
mkdir -p ~/.local/bin
curl -L https://github.com/rust-lang/rust-analyzer/releases/latest/download/rust-analyzer-x86_64-unknown-linux-gnu.gz | gunzip -c - > ~/.local/bin/rust-analyzer
chmod +x ~/.local/bin/rust-analyzer

references:
https://blog.logrocket.com/configuring-vim-rust-development/


## vs-code extention :
 Error Lens usernamehw
 
important package sqlx for postgres

$ install pkg-config openssl-devel / libssl-dev
$ cargo install sqlx-cli --no-default-features --features native-tls,postgres
$ sqlx migrate
$ cargo install cargo-watch
$ cargo watch -q -c -w src/ -x run
