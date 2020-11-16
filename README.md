# Charlatan
A very poor rustup-like program to install julia versions and tools

Charlatan is a small rust program that I've created to install the Julia
programming language in the current user `$HOME` directory without the need
of manual intervention.

:sweat_smile: I really don't recommend using this program in a serious way, 
since it's designed to suit my needs and not the community ones. However if this
abomination helps you in one way or another with your current Julia language
installations you are free to modify it at your own will and purpose.

## Why in Rust? :crab:
Because I was bored. I know this could be solved with a simple bash script. BUT
that would be boring asf. Also I wanted to practice Rust a little bit so this is
a good hobby while it lasts.


## Usage

To use this program you have 2 arguments

* `install`: This will install the Julia Programming language in your current
`$HOME` directory and depending on your choices it will also install extra tools
that might come in handy when developing Julia.

* `update`: This will attempt to update your current Julia installation.
