# Draft-RMS-Preprocessor
An exploratory project for designing a preprocessor for Aoe2 RMS files.

Features include:

- Removing comments.
- Minimizing whitespace.
- A `#BREAK` command for debugging to end a map script at a specific line.
- A `#REPEAT(N) ... #END_REPEAT` blocks for repeating code.
- Names for actor areas instead of numbers.
- Extraction of `rnd` to the beginning of the map script for debugging in combination with `#BREAK`.
- Various macros for generating lands in specific patterns, including placing lands in circles.
- `#HEADER_START ... #HEADER_END` blocks for emitting comments at the beginnings of output files.

Future features:

- Defining constants in terms of other constants, e.g. `BASE_TERRAIN GRASS` instead of `BASE_TERRAIN 0`.
- Further minifying repeated `create_land` commands by replacing the text `create_land` with a constant.
- Loop variables for `REPEAT` blocks.

## Some Notes on the Code

This project is the result of lots of ad-hoc changes I've needed to implement while writing map scripts.
It certainly is not a good example of well-written Rust code, or well-written code in general.
It's more a collection of experiments as I needed individual tasks automated and the cost of doing so was less than the cost of refactoring this library.

It started as Python script to remove comments for AoC maps (where comments inside of if statements were bugged, and especially caused difficulty when combining maps into map packs) and eventually morphed into this Rust program.
I never formalized a grammar and wrote parsing code by hand, so the map parsing code is quite messy and is very much not guaranteed to work on all map scripts.
In the future I'd like a full-blown LSP with better formatting, refactoring, linting, autocomplete, etc.
But this code base is not intended to morph into that.

For running the code, the `main` file has some hard-coded paths that will need to be setup for your machine.
The script reads maps from the given input directory and writes them to the local mod folder, without modifying the source files.
