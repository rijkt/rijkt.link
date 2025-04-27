
+++
title = "Setting up rijkt.link"
date = 2025-04-27
[taxonomies]
tags = ["meta"]
+++

This is a documentation post, for my future sanity's sake. If you too are interested in setting up your own website with an *SSG* like *Zola*, keep reading.  <!-- more --> All source code is available on [GitHub](https://github.com/rijkt/rijkt.link). 

## System Goals

I want to be able to write and publish text in a me-friendly way, as I am the only user. The content should be separated from the chosen presentation technology. There is no need for interactivity outside of basic navigation and maybe commenting. It should be possible to host everything on my own, to avoid vendor lock-in.

Since I recently switched my note-taking over from *org-mode* to Markdown, I would like the system to support it. I don't want to write inside an HTML file. The system should also be extensible and using a backend I'm interested learning, i.e. no *PHP*, *Node* or *Go*. 

I might want to publish some notes in German, others in English. Support for multiple languages would enable this more easily.

The design should be open for me to tinker with. I will not, however, design something from scratch. 

## Architecture



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
- Email

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

## Repository Setup

The *Git* repo is easily created using `zola init`. For easier building, the theme is located inside the repository as well. Instead of a package management system, the install is done via *Git* submodules. To add a new submodule run 

```
git submodule add <theme git repo> theme/<theme name>
```

When cloning the repository, the submodule has to be cloned as well. This is accomplished using the commands
```
 git submodule init
 git submodule update 
```
Versioning is handled with tags on the theme repository.
## Deployment
At time of writing, *Zola* is no longer properly supported for *Cloudflare Page's* *Git* repository integration. Trying to deploy with the `v2` build systems fails due to *Zola* being unavailable. Per the docs `v1` still supports *Zola*, but only up to version `0.14.0`. I'm currently running `0.20.0` locally. That version mismatch is a source of error's I'm not willing to debug if the worst comes to pass.

Automatic deployment via *Cloudflare* would have been convenient, but inverting the dependency here is nicer in practice. Therefore I simply deploy from a GitHub action. This still needs to be implemented at time of writing - [coming soon](https://github.com/rijkt/rijkt.link/issues/3).

## Credits

Reading [vale.rocks](https://vale.rocks/) and [gwern.net](https://gwern.net/) greatly inspired me to have a presence of my own on the web. [Zola](https://www.getzola.org/) and [Linkita](https://codeberg.org/salif/linkita) allowed me to get something off the ground quickly.