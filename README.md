# Crawl

`crawl` fetches recent items from multiple sources and prints them to the `stdout`. It can be used, for example, with Apple Shortcuts to deliver a newsletter periodically.

## Run

```sh
go run . \
  -atom 'Go Blog=https://go.dev/blog/feed.atom' \
  -gnews 'Associated Press=apnews.com' \
  -reddit 'Reddit Chess=chess' \
  -rss 'Hacker News=https://news.ycombinator.com/rss'
```
