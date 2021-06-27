[![CI](https://github.com/k-nasa/wai/actions/workflows/ci.yml/badge.svg)](https://github.com/k-nasa/wai/actions/workflows/ci.yml)

# wai (in progress)

A simple wasm interpreter

This is an ongoing project


## DEMO

https://user-images.githubusercontent.com/23740172/123530111-d8775280-d731-11eb-9ddf-b4afd640ccdb.mov


## Usage

```bash
wai examples/add.wasm --invoke add -a 1 2

wai examples/fib.wasm --invoke fib -a 10
```


```bash
:) % wai -h
wai 0.1.0
k-nasa <htilcs1115@gmail.com>
A simple wasm interpreter

USAGE:
    wai [OPTIONS] <file-path> --invoke <invoke>

ARGS:
    <file-path>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --args <args>...
    -i, --invoke <invoke>
```

## Licence

[MIT](https://github.com/k-nasa/wai/blob/master/LICENSE)
