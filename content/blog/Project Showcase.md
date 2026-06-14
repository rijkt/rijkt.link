+++
title = "Rust projects showcase"
date = 2026-06-12
[taxonomies]
tags = ["Rust"]
+++

## My Rust journey so far  <!-- more -->

As I have been getting deeper and deeper into my Rust journey and worked on numerous little projects, I wanted to take a step back. Let's take a look at what worked, what didn't and what I want to come back to in the future.

## The sound of Rust

The first "real" Rust I wrote was [rusty piano](https://codeberg.org/rijkt/rusty-piano). This was a collaborative effort with a friend who knew about the electrical side and me who knew about software. We experimented with synthesizing sound and outputting it to and old speaker. In our first attempt we used a raspberry pi and python, which felt wholly inadequate. Too may CPU cycles and a whole Linux installation wasted on a comparatively simple task. That would not do, therefore we opted to switch to an *Arduino Uno* running Rust. We ended up using [avr-hal](https://github.com/Rahix/avr-hal) but still had to implement some features according to the *atmega328p* data sheet. This was a lot of fun and showed me the advantages of using Rust in an embedded context, especially when using well-crafted hardware abstraction layers.

[![Video showcasing sound generation](https://pub-50a3834faf53422c806af488a9260701.r2.dev/blog/rust-piano-thumb.png)](https://pub-50a3834faf53422c806af488a9260701.r2.dev/blog/rusty-piano.mp4)


When we were satisfied with our *PWM* playground, I took a little break from Rust. My friend and I had new ideas and we went on to set up our shared backup solution we still use today. See [Backup Writeup](https://rijkt.link/blog/backup-writeup/) for more on that.

## Revisiting a childhood classic

While embedded programming was fun, it only gave a very narrow view of the language. Much work was done in the libraries used, so I mostly gained experience using those. I felt I hadn't seen what the language can do by itself. So I read through the book and got started with a pure Rust project: [minesweep-rs](https://codeberg.org/rijkt/minesweep-rs), a CLI minesweeper clone.

![Screenshot of a game of minesweeper in a terminal](https://pub-50a3834faf53422c806af488a9260701.r2.dev/blog/minesweep-rs.png)

As always with games, this one was a lot of fun to make and test. To challenge myself, I wanted to use no libraries outside std, where feasible. This way, I ended up getting over the initial steep learning curve that Rust is notorious for. Managing game state while also collecting user input will force you to get to know the borrow checker at least a little bit. I did separate those concerns with the hope of coming back and writing a solver at some point. I still want to do that, but before that I'll probably rewrite the whole thing to use *iced*. Sadly the rendering logic is not separated in the way that *iced* expects it to be, which will be a challenge in the next iteration.

## Getting dispatched by Uncle Bob
Allowing to swap out implementations was a big focus in the next project: A simple imageboard API, written using `axum` and `sqlx`: [fediboard](https://codeberg.org/rijkt/fediboard). The name stems from my initial plan to explore the ActivityPub protocol as well. I decided against it in the end though, because my focus landed elsewhere.

Coming from Java, I was used to dynamic dispatch being everywhere. Need to decide which code path to run? Add an interface with different behaviors in different implementations. Since nothing is set in stone at compile time, you could simply switch out an implementation when deploying your software. While this is powerful, it comes with a lot of drawbacks. Rust gives us the choice whether to do dynamic or static dispatch, each with their own advantages and drawbacks. 

Language design fascinates me, so for this project I challenged myself again. I wanted to implement a Clean Architecture style of separating concerns, while only using static dispatch to explore the limits of this approach. Just so we're clear, this is not a great idea and I do not recommend doing it this way. I just wanted to see where it lead me.

It took me [three months](https://codeberg.org/rijkt/fediboard/pulls/7) and a lot of experimentation, but I ended up with a  design for a REST API that works pretty well as a project template. I ended up coming to the same conclusions and going through the same iterations as described in this [article](https://tulipemoutarde.be/posts/2023-08-20-depencency-injection-rust-axum/#static-dispatch-against-a-struct), so I won't restate that here. It came down to using dynamic dispatch in specific circumstances and static dispatch by default. Going static all the way would have meant propagating a set of generic types through every function definition. Since dispatch is used here mostly as an architectural barrier between caller and callee, I chose pragmatism. There are very specific traits (e.g. `BoardPersistence`) at the architectural boundaries that use dynamic dispatch,. Everything else is static. Rust is out here, delivering the best of both worlds.

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