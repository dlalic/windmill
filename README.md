# Windmill

![Continuous integration](https://github.com/dlalic/windmill/workflows/Continuous%20integration/badge.svg)
[![CodeFactor](https://www.codefactor.io/repository/github/dlalic/resume/badge)](https://www.codefactor.io/repository/github/dlalic/windmill)

A game inspired by the [nice visualization](https://www.youtube.com/watch?v=M64HUIJFTZM) of a problem from the 52nd International
Mathematical Olympiad.

![](assets/windmill.gif)

## Problem

> Let `S` be a finite set of at least two points in the plane. Assume that no three points of `S` are
> collinear. By a windmill we mean a process as follows. Start with a line `l` going through a
> point `P ∈ S`. Rotate `l` clockwise around the pivot `P` until the line contains another point `Q`
> of `S`. The point `Q` now takes over as the new pivot. This process continues indefinitely, with
> the pivot always being a point from `S`.
> 
> Show that for a suitable `P ∈ S` and a suitable starting line `l` containing `P`, the resulting
> windmill will visit each point of `S` as a pivot infinitely often.

[Source and solution](https://www.imo-official.org/problems/IMO2011SL.pdf)

## Usage

`cargo run`


| Key | Action |
| :---: | --- |
| R | reset |
| U | increase speed |
| D | decrease speed |
| Esc | quit |

## Implementation

There are already a [handful](https://github.com/search?q=windmill+imo&type=Repositories) of open source implementations available in different languages and frameworks including the one from the visualization video. They're mostly implemented in a way that an angle between the line and the point is calculated and then compared to `0` or `PI` to detect new pivot. This implementation calculates point orientation in regards to the line and detects new pivot when orientation changes.