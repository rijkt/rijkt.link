+++
title = "Rust projects showcase"
date = 2026-06-23
[taxonomies]
tags = ["Rust"]
+++

My Rust journey so far.  <!-- more -->

# Rust projects showcase

As I have been getting deeper and deeper into my Rust journey and worked on numerous little projects, I wanted to take a step back. Let's take a look at what worked, what didn't and what I want to come back to in the future.

The first "real" Rust I wrote was [rusty piano](https://codeberg.org/rijkt/rusty-piano). This was a collaborative effort with a friend who knew about the electrical side and me who knew about software. We experimented with synthesizing sound and outputting it to and old speaker. In our first attempt we used a raspberry pi and python, which felt wholly inadequate. Too may cpu cycles and a whole linux installation wasted on a comparatively simple task. That would not do, therefore we opted to switch to an Arduino Uno running Rust. We ended up using [avr-hal](https://github.com/Rahix/avr-hal) but still had to implement some features according to the atmega328p data sheet. This was a lot of fun and showed me the advantages of using Rust in an embedded context, especially when using well-crafted hardware abstraction layers.

[![Video showcasing sound generation](https://pub-50a3834faf53422c806af488a9260701.r2.dev/blog/rust-piano-thumb.png)](https://pub-50a3834faf53422c806af488a9260701.r2.dev/blog/rusty-piano.mp4)


Up next:
- [minesweep-rs](https://codeberg.org/rijkt/minesweep-rs)
- [shovelhead-rs](https://codeberg.org/rijkt/shovelhead-rs)
- [fediboard](https://codeberg.org/rijkt/fediboard)
- [gentle-reminder](https://codeberg.org/rijkt/gentle-reminder)

<!-- 
- REST APIs with database integration.
- Async
- Polymorphism, how to work without it (and where I came crawling back to it).
- Clean Architecture
- little one off programs I use in hobbies (shovelhead-rs)
- game
-->