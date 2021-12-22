# Day 1

In part 1 we are asked to find the number of element form input that are bigger than the previous element.

In rust this is quite simple - we use `windows(2)` to get access to all such pairs, filter based on the `<` relation and count.

In part two we need to first collect sum of all subsequent triplets. After that we do the same algorith as in part 1.

I didn't make any optimization since in relesea mode straigthforward implementation runs in ~5.5µs.

# Day 2

In both parts we are asked to simulate movement of submarine. The only difference is slight change in the movement rules. Apart from that, this can be succintly expressed as a folding of list of moves with functions expressing movement rules.

I didn't make any optimization since in relesea mode straigthforward implementation runs in ~8.5µs.

# Day 3

In part 1 we are given set of binary sequences and need to find two new sequences. One is constructed form most common bits on each possition, the other from least common ones.

Part 2 is similar but after identifing most frequent bit at position N, we recurse to find remaining bits in subset of sequences that had that bit turned on. Similarly for least frequent bit at N, we find least frequent bit at N+1 by analysing subset of sequences that had N-th bit turned off. We do this until both sets have only one element.

In my implementation both parts reuse `most_frequent_bit`. I also did 2 optimizations. The bit sequences are represented using `SmallVec<[u8;12]>` - this avoids heap allocations but keeps the easy `Vec` like interface. The second optimization is the usage of `Vec::retain` which in part 2 lets me efficiently discard sequences that are no longer of interest.

Part 2 seems to have been first major filter and significatnly reduced number of participants.

Runs in ~92µs.

# Day 4

In day 4 we are given set of bing boards and a sequence of bingo numbers. We are asked to find first board that wins (part 1) and the last to win (part 2).

To make it fast, I'm representing each board as a map from the number to position on this board, number of marked position in each row and column and current score. Each time new number is marked, we check the map, and increment row and column in which it appears.

The map is implemented using `FxHashMap` which is a rust standard `HashMap` with simpler and faster (but less secure) hashing function. In fact this type is consistantly faster for my solutions and whenever I say map in future tasks I mean `FxHashMap` (or `FxHashSet`).

The runtime is around 50µs.

# Day 5

We are given set of lines defined as start and end points and are asked to count number of points in which at least 2 lines overlap. Part 1 asks as only about horizontal and vertical lines while part 2 adds in the diagonals.

First optimization is that the we can resuse part 1 and only "add" diagonals for part 2.

My actuall implementation uses 2d array (represented using flat array) of ints that is big enough to hold all points. Each line is "drawn" onto the array by incremnting all covered points while at the same time counting overlaps.

This makes it linear in the number of covered pixels which is much bigger than number of lines. Yet, the efficient implementation keeps the runtime below 1ms at 650µs.

# Day 6

This is first non trivial probelem to solve. We are given a population of animals of different age and the rulse that govern their reproduction. The task is to find the size of population after some time.

In part 1 the time is 80 days and we can simulate directly the reproduction process since the resulting population is small ~6000.

In part 2 the time is 256 and the result is ~10^10 - direct simulation will not work. To solve this we need to avoid storing each animal separately. Since the preproduction depends only on age and we have constrained set of possible ages (0-8) we can track mapping from ages to number of animals.

Using a simple array indexed by age keeps the runtime low, at 1.2µs.

# Day 7

Part 1 asks us to find in a given set of points, one which is closest to all other. This means minimization of sum of distances. This is just a median point.

Part 2 allows us to position not only on the points from input but also anywhere in between them. It also changes the cost to be a square of the distance. Simple loop from leftmost to rightmost point lets us calculate total costs. From that we select minimum.

Runtime is 23.5µs.

# Day 8

First task I didn't like. We basically need to find translation and are given enough constraint to deduce that translation. It's possible to work out the ruleo by hand and hardcode them. Instead my solution first detects words of unique lengths. This makes it possible to identify some of the letters. The rest is deduced by eliminating known letters from sets of possible letters - once new letter has only one candidate we remove that candidate from all other and this reveals next unique mapping.

Biggest runtime yet at 913µs.

# Day 9

First graph based probelm. In part 1 we need to find lowpoint on map. Eeasy to do - lookup all 8 surrounding points and check if the middle one is samllest.

In part 2 we need to find the size of the basin - expand the low points until reaching height 9 - simple BFS.

Another big runtime at 725µs

# Day 10

Parenthesis problem, which means stack. Part 1 asks us to check if parens match, which we do using stack - push on opening. On closing pop and verif that what was on top matches with closing one.

Part 2 asks us to find completion for correct but incomplete sequence, which is easy to derive from stack when we finish processing incomplet string.

Runtime is 30µs.

# Day 11

Somewhat game of life like problem with octopuses on a grid that can flash light and stimulate each other. Its enough to simulate exactly what the task requires - no tricks really needed.

Using smallvec the runtime is 440µs.