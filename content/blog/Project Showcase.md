+++
title = "Rust projects showcase"
date = 2026-06-12
[taxonomies]
tags = ["Rust"]
+++

My Rust journey so far.  <!-- more -->

# Rust projects showcase

As I have been getting deeper and deeper into my Rust journey and worked on numerous little projects, I wanted to take a step back. Let's take a look at what worked, what didn't and what I want to come back to in the future.

The first "real" Rust I wrote was [rusty piano](https://codeberg.org/rijkt/rusty-piano). This was a collaborative effort with a friend who knew about the electrical side and me who knew about software. We experimented with synthesizing sound and outputting it to and old speaker. In our first attempt we used a raspberry pi and python, which felt wholly inadequate. Too may CPU cycles and a whole Linux installation wasted on a comparatively simple task. That would not do, therefore we opted to switch to an *Arduino Uno* running Rust. We ended up using [avr-hal](https://github.com/Rahix/avr-hal) but still had to implement some features according to the *atmega328p* data sheet. This was a lot of fun and showed me the advantages of using Rust in an embedded context, especially when using well-crafted hardware abstraction layers.

[![Video showcasing sound generation](https://pub-50a3834faf53422c806af488a9260701.r2.dev/blog/rust-piano-thumb.png)](https://pub-50a3834faf53422c806af488a9260701.r2.dev/blog/rusty-piano.mp4)


When we were satisfied with our *PWM* playground, I took a little break from Rust. My friend and I had new ideas and we went on to set up our shared backup solution we still use today. See [Backup Writeup](https://rijkt.link/blog/backup-writeup/) for more on that.

While embedded programming was fun, it only gave a very narrow view of the language. Much work was done in the libraries used, so I mostly gained experience using those. I felt I hadn't seen what the language can do by itself. So I read through the book and got started with a pure Rust project: [minesweep-rs](https://codeberg.org/rijkt/minesweep-rs), a CLI minesweeper clone.

![Screenshot of a game of minesweeper in a terminal](https://pub-50a3834faf53422c806af488a9260701.r2.dev/blog/minesweep-rs.png)

As always with games, this one was a lot of fun to make and test. To challenge myself, I wanted to use no libraries outside std, where feasible. This way, I ended up getting over the initial steep learning curve that Rust is notorious for. Managing game state while also collecting user input will force you to get to know the borrow checker at least a little bit. I did separate those concerns with the hope of coming back and writing a solver at some point. I still want to do that, but before that I'll probably rewrite the whole thing to use Iced. Sadly the rendering logic is not separated in the way that iced expects it to be, which will be a challenge in the next iteration.
<!-- 
Allowing to change out implementations was a big focus in the next project, [fediboard](https://codeberg.org/rijkt/fediboard). Coming from Java, I was used to dynamic dispatch being used for everything. -->

Up next:
- [fediboard](https://codeberg.org/rijkt/fediboard)
- [shovelhead-rs](https://codeberg.org/rijkt/shovelhead-rs)
- [gentle-reminder](https://codeberg.org/rijkt/gentle-reminder)

<!-- 
- REST APIs with database integration.
- Async
- Polymorphism, how to work without it (and where I came crawling back to it).
- Clean Architecture
- little one off programs I use in hobbies (shovelhead-rs)
- game
-->
<!-- 
I want to come back to my first embedded project to add keys and really turn it into a piano. -->