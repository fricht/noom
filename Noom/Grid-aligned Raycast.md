# The problem

We need to send a ray from the player at an angle, and get the distance to the first hit object.

## Method 1 - Walking

[[Grid-aligned Raycast - Walking]]

This shitty af method didn't worked when i tried to implement it, although it worked on the paper.

You walk the line, stopping at each intersection with the grid, and checking if there is a block.

## Method 2 - Lines

[[Grid-aligned Raycast - Lines]]

Same algorithm as the one to [rasterize lines](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm), but checking each *pixel* if there is a block.
The tricky part is to get the actual distance.
