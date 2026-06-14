+++
title = "Rust projects showcase"
date = 2026-06-12
[taxonomies]
tags = ["Rust"]
+++

## My Rust journey so far  <!-- more -->

As I have been getting deeper and deeper into my Rust journey and worked on numerous little projects, I wanted to take a step back. Let's take a look at what worked, what didn't and what I want to come back to in the future.

## The sound of Rust

The first "real" Rust I wrote was [rusty piano](https://codeberg.org/rijkt/rusty-piano). This was a collaborative effort with a friend who knew about the electrical side of things. We experimented with synthesizing sound and outputting it to an old speaker. In our first attempt we used a *Raspberry Pi* and *Python*, which felt excessive. Too may CPU cycles and a whole Linux installation wasted on a comparatively simple task. That would not do, therefore we opted to switch to an *Arduino Uno* running Rust. We ended up using [avr-hal](https://github.com/Rahix/avr-hal) but still had to implement some features according to the *atmega328p* data sheet. This was a lot of fun and showed me the advantages of using Rust in an embedded context, especially when using well-crafted hardware abstraction layers.

[![Video showcasing sound generation](https://pub-50a3834faf53422c806af488a9260701.r2.dev/blog/rust-piano-thumb.png)](https://pub-50a3834faf53422c806af488a9260701.r2.dev/blog/rusty-piano.mp4)


When we were satisfied with our *PWM* playground, I took a little break from Rust. My friend and I had new ideas and we went on to set up our shared backup solution we still use today. See [Backup Writeup](https://rijkt.link/blog/backup-writeup/) if you'd like to know more.

## Revisiting a childhood classic

While embedded programming was rewarding, it only gave me a very narrow view of the language. Much work was done in the libraries used, so I mostly gained experience using those. I felt I hadn't seen what the language can do by itself. So I read through the book and got started with a pure Rust project: [minesweep-rs](https://codeberg.org/rijkt/minesweep-rs), a CLI minesweeper clone.

![Screenshot of a game of minesweeper in a terminal](https://pub-50a3834faf53422c806af488a9260701.r2.dev/blog/minesweep-rs.png)

As always with games, this one was a lot of fun to make and test. To challenge myself, I wanted to use no libraries outside std, where feasible. This way, I ended up getting over the initial steep learning curve that Rust is notorious for. Managing game state while also collecting user input will force you to get to know the borrow checker at least a little bit. I did separate those concerns with the hope of coming back and writing a solver at some point. I still want to do that, but I'll probably have to rewrite the whole thing to use a nicer frontend. Sadly, swapping out the rendering component would have taken some more architecting. The rendering logic is not separated in the way that e.g. *iced* expects it to be, which will be a challenge in the next iteration.

## Getting dispatched by Uncle Bob
Allowing to swap out implementations was a big focus in the next project: A simple imageboard API, written using `axum` and `sqlx`: [fediboard](https://codeberg.org/rijkt/fediboard). The name stems from my initial plan to explore the ActivityPub protocol as well. I decided against it in the end though, because my focus landed elsewhere.

Coming from Java, I was used to dynamic dispatch being everywhere. Need to decide which code path to run? Add an interface with different behaviors in different implementations. Since nothing is set in stone at compile time, you could simply switch out an implementation when deploying your software. While this is powerful, it comes with a lot of drawbacks. Rust gives us the choice whether to do dynamic or static dispatch, each with their own advantages and drawbacks. 

Language design fascinates me, so for this project I challenged myself again. I wanted to implement a Clean Architecture style of separating concerns, while only using static dispatch to explore the limits of this approach. Just so we're clear, this is not a great idea and I do not recommend doing it this way. I just wanted to see where it lead me.

It took me [three months](https://codeberg.org/rijkt/fediboard/pulls/7) and a lot of experimentation, but I ended up with a  design for a REST API that works pretty well as a project template. I ended up coming to the same conclusions and going through the same iterations described in this [article](https://tulipemoutarde.be/posts/2023-08-20-depencency-injection-rust-axum/#static-dispatch-against-a-struct), so I won't restate that here. It came down to using dynamic dispatch in specific circumstances and static dispatch by default. Going static all the way would have meant propagating a set of generic types through every function definition. Since dispatch is used here mostly as an architectural barrier between caller and callee, I chose pragmatism. There are very specific traits (e.g. `BoardPersistence`) defined at architectural boundaries where dynamic dispatch makes sense. Everything else is static. 

The great part about this to me is that we have the ability to choose. Rust is out here, delivering the best of both worlds.

## A period piece

Next up we have my current project: [gentle-reminder](https://codeberg.org/rijkt/gentle-reminder), a self-hostable reminder service. It is again written using `axum` and based on the the project template described above. This time I am integrating external services as well, for example by sending reminders by e-mail. With this project I am aiming to clear out my calendar a bit and receive periodic reminders via different channels.

The challenge here was managing the threads responsible for sending triggered reminders. Since they can be dynamically added and removed, some scheduling is needed. Thankfully *tokio* provides the `CancellationToken` abstraction, which made this pretty easy in the end. Add some basic event handling after deletes and updates and voilà: A reminder e-mail coming your way.

Next up is a frontend. My APIs already ship OpenAPI endpoints and I've been itching to generate some Webassembly. 

## Reflections

Every project helped me understand and refine core Rust concepts. There was, of course, a lot that didn't work. Arguing with with the borrow checker and deciphering seemingly incomprehensible type errors were daily activities. 

To overcome complexity, the best tool in my belt has always been writing. When I am truly stumped, I open up a document and interrogate my assumptions about the problem at hand. I gather documentation and error messages until I synthesize the solution.  I am still often reminded of those notes and refer back to them in my daily programming. 

By now I have a good grasp on Rust and its ecosystem. I've been getting deep into async Rust and web server tech. I'm hyped for new developments and am keeping up with Rust RFCs like [RTN](https://github.com/rust-lang/rust/issues/109417) (here's hoping).

Writing Rust for the web is a lot of fun. Nonetheless I would like to return to my embedded project some day and finish that piano.
