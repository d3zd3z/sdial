# Master Lock Speed Dial(r) lock simulation

Master Lock's Speed Dial(r) combination lock is a readily available,
affordable, and reasonably secure combination lock.  I wanted to
better understand the combination space of this lock (and just its
functionality) a little better.

This simulator exhausts the combination space and can give information
about combinations that more or fewer conflicts and might give some
hints on what good combination choices would be.

Please see [this
visualizer](https://toool.nl/images/e/e1/MhVisualizer_V2.0_p.swf)
and [this
paper](https://toool.nl/images/e/e5/The_New_Master_Lock_Combination_Padlock_V2.0.pdf)
to help understand how the lock actually works.

## Basic use

```
cargo run --release
...
For up to 10 moves
7396 Uniques
1390704 dups
Best: (0|,2>,3<,3<) (1 target) (URRLLU)
```

The `--release` is important for performance.  By default, the program
will simulate all combinations up to 10 moves, print some statistics,
and arbitrarily print information about the "best" combination.

The information printed is first the state of the 4 wheels in the lock
after applying this combination.  Any sequence of moves resulting in
this state would be able to open a lock set to this combination.

To get more information, you can give arguments to the program,
delimiting from the cargo command with two hyphens:

```
cargo run --release -- --help
```

## Good combinations.

Running with `--all` clearly shows that some combinations are better
than others (there being four states that can be reached with 1,985
different up-to-10-move sequences.  The lock is radially symmetric, so
anything found can be applied to a different combination by rotating
the lock a multiple on 90 degrees).

However, if the attacker knows that the user has used this simulation
to choose a combination, they might start with those combinations that
result in the fewest collisions.

It would seem, then, that the best choice would be to compromise
between these, and choose a combination that has a small, but not too
small, number of collisions, but whose shortest reaching sequence is
as long as the user can tolerate memorizing.  It would be silly to
choose a 10-step combination if there is a 5-step combination that
reaches the same target state.
