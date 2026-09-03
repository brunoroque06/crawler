# Feed

`feed` fetches recent items from multiple sources and prints them to the `stdout`. It can be used, for example, with Apple Shortcuts to deliver a newsletter periodically.

## Run

```sh
cargo run -- \
  --atom 'Rust Blog=https://blog.rust-lang.org/feed.xml' \
  --gnews 'Associated Press=apnews.com' \
  --reddit 'Reddit Chess=chess' \
  --rss 'Hacker News=https://news.ycombinator.com/rss'
```
