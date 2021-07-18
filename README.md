# Random matrix eigenvalue toy

## What is this about?

Based on https://twitter.com/sigfpe/status/984889330467352576, I
learnt about https://en.wikipedia.org/wiki/Circular_law, which seems
pretty cool. I wanted to play about with it myself, and perhaps
understand the way in which the points kind of repel each other.

It's also a nice excuse to do a little more Rust and some
computational linear algebra (aka use the `nalgebra` library).

## Getting started

```
cargo +nightly run eigenvalues
```

## In more detail

I first tried producing an animation of what happens as you increase
the size of the matrix, adding more eigenvalues (more specifically, it
plotted the eigenvalues as I took n x n sub-matrices of the full-sized
matrix). This just produced a pattern of eigenvalues bobbling about a
bit, and gave no real insight.

I then tried interpolating between a matrix where the last eigenvector
is orthogonal to all the others (forced by having the last dimension
of matrix be diagonal-element only) and the full random matrix. This
should effectively move between the eigenvalues of the n-1 x n-1
matrix and the eigenvalues of the n x n matrix, so you can see the
other eigenvalues adjust to deal with the new eigenvalue. The mean and
variance will also slightly change, which I've not accounted for, but
at high dimension I hope the effect is minor.

TBH, the resulting animation is hard to interpret, too!
