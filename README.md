# neon-talk

> [View the slides][slides]

## Installation

1. Install [Rust][rust]
1. Install [Node][node]
1. Install all [node-gyp][node-gyp] dependencies

## Building

### Production

```sh
npm install
# or
npm run build --production
```

### Development

```sh
npm run build
# or to type check
cargo check
```

## Benchmarks

### JS

Benchmark methods directly.

```sh
npm run bench
```

#### Results

```
Javascript x 13.58 ops/sec ±7.06% (37 runs sampled)
Rust Sync x 139 ops/sec ±5.57% (69 runs sampled)
Rust Async x 148 ops/sec ±6.46% (65 runs sampled)
Fastest is Rust Async
```

### Server

Benchmark express middleware with [Apache Bench][ab].

```sh
./ab.sh
```

#### Results

```
Js    Sync Results: Requests per second:  55.18 [#/sec] (mean)
Rust Async Results: Requests per second: 670.34 [#/sec] (mean)
```

[slides]: https://kjvalencik.github.io/neon-talk/
[rust]: https://www.rust-lang.org
[node]: https://nodejs.org
[node-gyp]: https://github.com/nodejs/node-gyp
[ab]: https://httpd.apache.org/docs/2.4/programs/ab.html
