
+++
title = "Setting up rijkt.link"
date = 2025-04-26
[taxonomies]
tags = ["meta"]
+++


This is a documentation post, for my future sanity's sake. If you too are interested in setting up your own website with an *SSG* like *zola*, keep reading. All source code is available on [GitHub](https://github.com/rijkt/rijkt.link).

## Terms

- SSG
- Zola
- Markdown
- Kita
- Linkita
- Git submodules
- Cloudflare
- Cloudflare Pages
- Cloudflare Domain
- Cloudflare Proxy
- Github, Github Actions

{% mermaid() %}
architecture-beta
    group api(cloud)[API]

    service db(database)[Database] in api
    service disk1(disk)[Storage] in api
    service disk2(disk)[Storage] in api
    service server(server)[Server] in api

    db:L -- R:server
    disk1:T -- B:server
    disk2:T -- B:db
{% end %}
