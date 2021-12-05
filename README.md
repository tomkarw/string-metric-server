# String Metric Server
## Intro
A simple websocket server that calculates distance between two strings in a couple different metrics.
It runs a static site that upgrades to websocket connection and allows to input two strings.
All connected users will receive the result of the query.

Currently, implemented metrics are:
* [Hamming distance](https://en.wikipedia.org/wiki/Hamming_distance)
* [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance)
* [Jaro distance](https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance) (technically not a metric)


## Usage

Run server on `127.0.0.1:8080` by default:
```
cargo run
```
and go to 127.0.0.1:8080.


You can also specify the port:
```
cargo run -- 80
```

## Resources
The server was heavily influenced by [warp websocket chat example](https://github.com/seanmonstar/warp/blob/master/examples/websockets_chat.rs).

Jaro distance was ported from [C++ implementation](https://www.geeksforgeeks.org/jaro-and-jaro-winkler-similarity/) on Geeks for Geeks.
