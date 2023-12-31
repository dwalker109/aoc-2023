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

## Day 8: Haunted Wasteland

A maths one. I find these a bit frustrating because my maths is waaaaaaaay to weak to
work out patterns like the one required here - LCM. Part 1 was fine, and my part 2 was
a reasonable extension of that, but it never completes because you really need the maths
trick to do it. I had to look at hints, which feels cheap. I used a lib to calculate LCM
because I'd just be copying an impl verbatim anyway, and this is probably what I'd do
in the real world tbh.

## Day 9: Mirage Maintenance

Ultimately, a fairly straightforward business of mapping/folding number sequences.
I wasted a fair bit of time going from test to real input, because I was calculating
*absolute* differences, not *relative* differences. Once I fixed that it was plain
sailing. Part 2 was incredibly simple. My code is a bit janky but it runs fast
so meh.

## Day 10: Pipe Maze

This was torrid. Part 1 I managed in an hour or two, and I was pleased with myself,
Working out the right pipe segment was challenging and satisfying. And then part 2.
This was a nightmare. I drew a blank, checked Reddit for hints and learned that a 
line by line single scan would allow you to work this out. *Most* of this was obvious
from looking at the sample data (walls flipping between inner and outer states) but
getting the corners right was nasty trial and error and I didn't understand why it eventually
worked. After submitting my answer I looked into this more and found some decent 
explanations, which I have included in the source as a reminder. It also turns out 
I made an error in one of my sample inputs, which made it all much harder.

## Day 11: Cosmic Expansion

Should have been simple. Part 1 I completed nicely in about an hour, and I used
an offset approach for co-ordinates (rather than mapping out all the emptiness
in memory) mainly because it seemed nicer. Turns out that is exactly what part 2
needed. However, I lost far more time than I would like to admit in working out
why part 1 passed and part 2 didn't. Turns out I was ADDING the empty space I needed
all along, not REPLACING it. Once I fixed that we're good. There's a lot of HashMap
lookups in a hot loop so I used FxHashMap to get the speed down to about 4ms per part.

## Day 13: Point of Incidence

Pretty simple. Part 1 came together in not much time, part 2 is not much more. 
Got the runtime down to less that half a millisecond by avoiding collections and
working with byte refs. Trying to use bitvecs made no difference so scrapped that.

## Day 14: Parabolic Reflector Dish

A cycle detection problem, similar to the Tetris nightmare from 2022. That one 
prepared me though and I didn't find this one too bad. Actually figuring out the 
whole modulo things to decide how far to fast-forward I had to poke at until it
worked, but I pretty much intuited it this time which was satisfying. My initial
impl was incredibly slow though, taking 30 seconds to complete on an M1 MacBook Pro.
After optimising it runs in under a second, which is fast enough for now.

## Day 15: Lens Library

A pleasant, straightforward, follow the steps kinda day. The hardest thing about this
one was following the text and realising that the `dbg!` output and RustRover debugger
both present slightly confusing information about `VecDeque`. Plus a few small bugs
about text parsing which made it seem trickier that it was. Runs very fast though.

## Day 16: The Floor Will Be Lava

This day really ended up being a painful chore, but it wasn't even difficult. Read 
the steps, write some path following code, easy. I had most of it written in an hour.
But I then spent the next 5 hours debugging an incredibly silly bug which failed on
real input, but not the tests. I eventually rewrote the path following and it all
fell into place. Part 2 was just a brute force, and with Rayon deployed takes about
100ms. My code is kinda smelly but I'm done with this one.

## Day 18: Lavaduct Lagoon

Part 1 is a naive walk the steps and flood fill, which I (along with everybody else)
was skillfully misdirected into because of some super important hex colour data stored
with it. I kept it as is for posterity, Part 2 then revealed itself as a planet scale 
geometry problem, far too big to brute force. I decided to use the `geo` crate instead 
of writing this myself, which seemed like a much more fun approach than copying an 
algorithm off the internet. It worked really well, though I did need help in adding 
the final piece of the puzzle (appending some missing area data which comes from the 
fact that our area calculation isn't working on a 1 unit wide trench, but rather a 
zero area set of lines).

## Day 19: Aplenty

Thoroughly enjoyable day. Messed around with closures rather than enums for my rule
processing logic mainly for giggles, since enums probably would have meant for less
type gore. Part 2 built on the range narrowing we did in day 5 really well, so I 
enjoyed that one (even tracking down the small off by 1 introduced errors). I ended
up with a totally separate implementation for part 2, since while I probably could 
have adapted it into one solver, I wrote it discretely and will leave it like that.

## Day 20: Pulse Propagation

Mixed feelings about this one. Set up a load of modules implementing a trait, which 
worked really well for part 1 (once I stopped trying to use mpsc channels and just
went for a VecDequeue based impl). Part 2 involved finding a LCM, and while I 
was fairly sure straight away that is what I would be looking for, I couldn't
figure out what I was looking for. After looking at the hints I found a solution
very quickly, and it was pretty simple.

