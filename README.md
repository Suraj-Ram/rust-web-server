# Multithreaded Web Server

Web server that can handle requests simultaneously using a custom thread pool of 4 threads.

To run
```bash
cargo run
```

- `/` is an endpoint that returns a simple HTML page with a welcome message.
- `/sleep` is an endpoint that sleeps for 5 seconds before returning a simple HTML page with a welcome message.
  - This is used to benchmark the server's ability to handle multiple requests simultaneously.

To benchmark the server, run `./benchmark.sh` or use [hyperfine](https://github.com/sharkdp/hyperfine) as follows
```bash
hyperfine "./benchmark.sh" --runs 5
```
