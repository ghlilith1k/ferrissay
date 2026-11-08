# ferrissay

a cowsay remake using Rust's mascot "Ferris", written in Rust, obviously

<p align="center">
  <img src="https://github.com/ghlilith1k/ferrissay/blob/main/assets/ferrissay.png" alt="ferrissay preview"/>
</p>

Copyright (c) 2026 ghlilith1k - GNU/GPLv2 or later

# Description

ferrissay is just a remake of the popular linux CLI program "cowsay",
the user types a word or a phrase after the command and an ASCII art
of Ferris appears with a cloud floating above him saying the word/phrase
the user typed, just like cowsay but with a crab, pretty simple no?
it's still in early development but it already works, there might be some
visual bugs for now but i'm gonna fix them in the future.
**bugs are expected, please report them in the "issues" tab**

# How to Install:

    make

# Usage:

    ferrissay [-u] [message]

# About

**note: ferrissay has only been tested on GNU/linux, stability/full functionality and compilation on other OSes is not guaranteed.**

- current version: v0.1.1-beta
- current stable: none
- total lines of code: 125
- for future releases: see "Dev Notes"

# Dev Notes / Known Issues

- fix text box glitching when phrase gets really long
- make Ferris prettier
- make new Ferris variations and enable them with flags
- the whole program overflows if a word contains the -u argument attached, i don't know why
- improve argument parsing logic
- ferrissay hangs when -u is passed but message is empty
- modify both visual and code design in the future

# Changelog

## release v0.1.0-beta - packaged

just the standard program, starting point for future releases

## release v0.1.1-beta - packaged

- added ferries.claws_down and made it default
- added very basic argument parsing logic
- modified text box design
- switched from free variables to struct
- fixed first-argument bug, now the command itself doesn't get printed
- formatted code
- some other small fixes...
