# Day 02

## Part 1

The elves are running low on wrapping paper, and so they need to submit an order for more. They have a
list of the dimensions (length l, width w, and height h) of each present, and only want to order
exactly as much as they need.

Fortunately, every present is a box (a perfect right rectangular prism), which makes calculating
the required wrapping paper for each gift a little easier: find the surface area of the box,
which is ![equation](https://latex.codecogs.com/svg.image?\bg{white}2&space;\times&space;l&space;\times&space;w&space;&plus;&space;2&space;\times&space;w&space;\times&space;h&space;&plus;&space;2&space;\times&space;h&space;\times&space;l).
The elves also need a little extra paper for each present: the
area of the smallest side.

For example:

- A present with dimensions 2x3x4 requires ![equation](https://latex.codecogs.com/svg.image?\bg{white}2&space;\times&space;6&space;&plus;&space;2&space;\times12&space;&plus;&space;2&space;\times&space;8&space;=&space;52&space;\textrm{&space;ft}^2)
of wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
- A present with dimensions 1x1x10 requires ![equation](https://latex.codecogs.com/svg.image?\bg{white}2&space;\times&space;1&space;&plus;&space;2&space;\times&space;10&space;&plus;&space;2&space;\times&space;10&space;=&space;42&space;\textrm{&space;ft}^2)
of wrapping paper plus 1 square foot of slack, for a total of 43 square feet.

All numbers in the elves' list are in feet. How many total square feet of wrapping paper should they order?
