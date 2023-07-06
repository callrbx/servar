# Servar - Multi Utility Server

## Goals
Servar is eventually designed to be able to replace:
 - netcat
 - socat
 - python's HTTP serve
And pretty much any other utility you can or want to hook up to the network.
Our design goal is modularity and cross compatibility; this tool should work as well on any platform that can compile Rust.

While combining as many tools as possible, we are aiming to keep the flags at least very similar.

We are also not trying to steal the spotlight of any particular tool. Although we are trying to cram a lot of the "core" functionalities of various tools, we are not trying to replicate all the specialized features (at least for now). Inch deep, mile wide.

## Usage
Servar is designed to be very simple to use, with most of the flexibility coming from the sub-modules. By using Rust's `structopt` library we are able to provide detailed yet flexible help menus. 

```
USAGE:
    servar [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --ip <ip>        Listen IP [default: 0.0.0.0]
    -p, --port <port>    Listen Port [default: 8000]

SUBCOMMANDS:
    help        Prints this message or the help of the given subcommand(s)
    http-dir    Serves a directory via HTTP
```

To serve the /tmp directory over HTTP on port 8080:
`servar -p 8080 htt-dir /tmp`

## Results
Currently, only the HTTP side is supported. Here is a simple benchmark using `wrk` that compares the python module (top results) to the servar (bottom):

```
❯ ./wrk -t12 -c400 -d30s http://localhost:8000/
Running 30s test @ http://localhost:8000/
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    10.02ms   61.32ms   1.82s    97.98%
    Req/Sec   300.42    239.27     1.60k    67.01%
  86753 requests in 30.09s, 55.10MB read
  Socket errors: connect 0, read 0, write 0, timeout 44
Requests/sec:   2883.49
Transfer/sec:      1.83MB
❯ ./wrk -t12 -c400 -d30s http://localhost:8000/
Running 30s test @ http://localhost:8000/
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     6.50ms    4.34ms 211.01ms   73.87%
    Req/Sec     5.30k   677.75    13.70k    96.56%
  1899670 requests in 30.10s, 1.04GB read
Requests/sec:  63111.88
Transfer/sec:     35.51MB
```

As you can see, over the same 30s period, servar was able to handle more than double the ammount of connections, at almost 2x the speed.
