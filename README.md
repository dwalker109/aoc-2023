# Advent of Code 2023!

Another year, another wild ride from pleasant one liners through to enjoyable small challenges
through to head scratchers through to oh God when will it end through to yeah maybe finish this
one later in the year when I'm not so tired.

I'm using Rust again. Just because I still really enjoy writing it, and still have much to learn.
Dependencies are allowed this year, within reason - nom, serde, rand etc. Why not.

# New dawn, new day

- Each day can now be boilerplated via [cargo-generate](https://github.com/dwalker109/aoc-input).
- [aocfin](https://github.com/dwalker109/aoc-input) can be used to pull the day's input.
- Inputs are stored in a [private submodule](https://github.com/dwalker109/aoc-input) mounted under `./input`, pointing at the `aoc-2023` branch.

# Days!

## Day 1: Trebuchet?!

This was a surprisingly involved first day. Is 2023 going to be hard as nails?
It wasn't massively difficult, and maybe I'm just still asleep (I got up at 5am),
but this took me an hour. Day 1 is usually pretty much a one liner, and this one
wasn't.

## Day 2: Cube Conundrum

Mainly about parsing text. I decided not to reach for nom yet. Manually parsing it
felt a bit grubby but tbh it worked out ok and I'm reasonably happy with what I 
ended up with. 

## Day 3: Gear Ratios

A fairly standard Xy lookup type problem, made more difficult by sequences of
some items counting as one item. I could have done some data cleanup to make this
somewhat trivial but I went with something a bit more refined/fussy in the end. 
There's a fair bit of code but it's OK.

## Day 4: Scratchcards

A relaxed day, welcome after a more tricky weekend start than expected. Fairly
straightforward comparison of lists and calculating scores. Part 2 was made a little
more tricky with a more complicated scoring system, but probably the easiest day so far.

## Day 5: If You Give A Seed A Fertilizer

Well, this one was challenging. Part 1 was easy, and I actually used my part 1 to 
bruteforce part 2 on the day, which ran for about 3 minutes and gave me the right answer!
This wasn't good enough though, so I followed some of the Reddit advice about splitting
into ranges and eventually implemented a solution which runs in less than a millisecond.

## Day 6: Wait For It

Err... easiest day by far. Expected part 2 to make the stupid naive part 1 solution
untenable, but is didn't. Completes in 20 ms. Weird, but welcome tbh. Ha!

## Day 7: Camel Cards

Again, not too tricky and ultimately satisfying. After parsing the hands into structs,
implementing the Ord/PartialOrd traits on them makes sorting hands easy. Part two
necessitated a few tweaks but a simple "mode" enum made those easy to implement too.
