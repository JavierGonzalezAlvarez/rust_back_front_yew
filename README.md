# writing the back and the front ni rust with activeweb and yew

# back with activex-web & posgres
$ cargo --vesion
$ cargo new rust_api_back_front --bin

## check code
cargo check

## performance test
wrk =>  a web url performance test tool 
$ wrk -t8 -c256 -d30s http://127.0.0.1:8080/

## endpoints
get => http://127.0.0.1:8000/get_all
post => http://127.0.0.1:8000/post_one

# front with yew 
$ cargo new front

## dependencies
$ rustup target add wasm32-unknown-unknown
$ cargo install trunk

## run app
$ trunk serve

http://0.0.0.0:8080/