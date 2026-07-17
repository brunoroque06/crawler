package main

import (
	"flag"
	"os"
	"strings"
)

var (
	last   = flag.Int("last", 8, "items per feed")
	cutoff = flag.Int("cutoff", 25, "ignore older than, in hours")
)

var sources = []struct {
	name  string
	usage string
	build func(value string) source
}{
	{"atom", "name=url", func(v string) source { return atom{url: v} }},
	{"gnews", "name=query", func(v string) source { return gnews{query: v} }},
	{"reddit", "name=sub", func(v string) source { return reddit{sub: v} }},
	{"rss", "name=url", func(v string) source { return rss{url: v} }},
}

func parseFeeds() []feed {
	var feeds []feed

	for _, def := range sources {
		flag.Func(def.name, def.usage+" (repeatable)", func(s string) error {
			name, val, ok := strings.Cut(s, "=")
			if !ok {
				return errorf("expected %s, got %q", def.usage, s)
			}
			feeds = append(feeds, feed{name: name, src: def.build(val)})
			return nil
		})
	}
	flag.Parse()

	if len(feeds) == 0 {
		stderr("no feeds specified")
		flag.Usage()
		os.Exit(1)
	}

	return feeds
}
