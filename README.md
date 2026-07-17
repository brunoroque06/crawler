# Crawl

`crawl` fetches recent items from multiple feed sources and prints them to `stdout`. For example, it can we used with Apple Shortcuts to deliver a newsletter periodically.

## Run

```sh
go run . \
  -atom 'Go Blog=https://go.dev/blog/feed.atom' \
  -gnews 'Associated Press=apnews.com' \
  -reddit 'Reddit Chess=chess' \
  -rss 'Hacker News=https://news.ycombinator.com/rss'
```
