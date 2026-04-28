# Advent Of Code 2023

My solutions for Advent of Code 2023 implemented in Rust. The development environment is VS Code and Windows.

## Day 1

First introduction to Rust. My first impression is that the pervasive use of Option<> makes even the most trivial tasks, such as this one, very complicated.

| Part | Answer |
|-----:|-------:|
|    1 |  54331 |
|    2 |  54518 |

## Day 2

Copilot is turning out to be relatively useful. It is pretty good at doing the boilerplate code and copy pasta. The boilerplate code is  especially helpful while Rust is new to me. I don't have to worry about all of the minute details.

| Part | Answer |
|-----:|-------:|
|    1 |   2593 |
|    2 |  54699 |

## Day 3

Collections in Rust are pretty straightforward. Objects are still a bit of a mystery.

| Part |  Answer  |
|-----:|---------:|
|    1 |   539637 |
|    2 | 82818007 |

## Day 4

Objects are still a mystery -- object vs. reference and move vs. copy, and wtf is borrowing?. Rust and copilot are helpful, telling me how to fix the problems, but I suspect there are many times I am making copies of objects when I could just be using references.

| Part |  Answer |
|-----:|--------:|
|    1 |   25231 |
|    2 | 9721255 |

## Day 5

Well, that escalated quickly! Part 1 is simple. Part 2 shows that it wasn't a scalable solution. Manipulating intervals is interesting stuff. I learned there is something called an interval tree, but I don't think it has any use here.

| Part |   Answer   |
|-----:|-----------:|
|    1 | 1181555926 |
|    2 |   37806486 |

## Day 6

Pretty trivial. I can imagine someone iterating through the possibilities, but the solution for both parts is to solve the quadratic equation and the answer is simply the difference between the roots.

| Part |  Answer  |
|-----:|---------:|
|    1 |   114400 |
|    2 | 21039729 |

## Day 7

Pretty trivial again. Lack of good test data made debugging difficult. Speaking of debugging, I cannot figure out how to debug with command line arguments.

| Part |  Answer   |
|-----:|----------:|
|    1 | 250232501 |
|    2 | 249138943 |

## Day 8

Again pretty trivial, but part 2 involved some manual work in order to discover the cycles and determine their lengths, and then to deal with fact that they are not prime.

As for Rust, HashMap is convenient, but unlike C/C++, `String` is not `str` and not `Vec<char>`.

Also, a function returning a reference requires specifying the lifetime of the reference, and it is not clear that you can't specify the wrong lifetime. For example, I returned a reference to a string in an element of a HashMap and specified (I believe) that the reference's lifetime is the same as the HashMap, but that is not correct, is it? Does the lifetime of a HashMap end when an element is removed from it, or does my specification ensure that the reference String is not destroyed until the HashMap is destroyed?

| Part |     Answer     |
|-----:|---------------:|
|    1 |          19631 |
|    2 | 21003205388413 |

## Day 9

Again trivial. In Rust, you have to make a conscious effort to avoid making copies of objects.

| Part |    Answer  |
|-----:|-----------:|
|    1 | 1743490457 |
|    2 |       1053 |

## Day 10

Pretty easy. It helps that I know how to determine if a point is inside a polygon.

| Part | Answer |
|-----:|-------:|
|    1 |   6909 |
|    2 |    461 |

## Day 11

Easy again. I wish Rust weren't so picky about integer types. I'm using `as` everywhere, so it is not really helping. Copilot has been really helpful by writing the glue and boiler plate code for me, and also showing how certain things can be implemented in Rust.

| Part |    Answer    |
|-----:|-------------:|
|    1 |     10033566 |
|    2 | 560822911938 |

## Day 12

Permutations ... It could get ugly, but otherwise it is fairly straightforward with a brute force approach.

In part 1, I converted text to binary to hopefully boost performance. Basically, the brute force approach works ok.

In part 2, I had to move to 128-bit integers, but as I expected, the brute force approach doesn't scale sufficiently. I'm going to have to do something different. I gave up and looked up the trick -- memoization. Duh!

| Part |     Answer     |
|-----:|---------------:|
|    1 |           7716 |
|    2 | 18716325559999 |

## Day 13

Easy again. I am learning about the features of Rust by asking Copilot to improve my code. `if let ...` is interesting. `if let Some(x) = ...` is especially handy.

| Part | Answer |
|-----:|-------:|
|    1 |  34202 |
|    2 |  34230 |

## Day 14

Easy again, though I think I could have improved performance so part 2 wouldn't take so long. Instead of moving the rocks 1 cell at a time, I computed the destination for each rock and then swapped it. I think another optimization would be to find the number of rocks in each span and then just assign them to the correct spots and clear the rest. The cost for each span would be 2 scans rather than 1 scan plus 1 scan per rock.

Anyway, I discovered that in part 2, the load repeats every 360 cycles, so I only have to do 1,000,000,000 % 360 cycles to get the answer.

| Part | Answer |
|-----:|-------:|
|    1 | 106648 |
|    2 |  87700 |

## Day 15

I don't know. These challenges have been very straightforward thus far (except for day 12 and day 14). They seem simpler than previous years. However, I am only halfway through...

I spent a lot of time unsuccessfully trying to figure out how to get a working and concise equivalent of this: `let boxes: Vec<Vec<Lens>> = vec![Vec:new(); 256];`. I imagine someone that knows Rust well would say, "Oh, yeah. You can't".

It turns out that conditional compilation is cumbersome. I needed it this time because parts 1 and 2 are completely different and I don't know how to manage 2 executables in the same project ("crate"?).

| Part | Answer |
|-----:|-------:|
|    1 | 506891 |
|    2 | 230462 |

## Day 16

Trivial. Took the chance to organize, refactor and clean up some code. I decided to use a stack instead of a recursive implementation. Recursive code is generally harder to read and debug.

| Part | Answer |
|-----:|-------:|
|    1 |   7477 |
|    2 |   7853 |

## Day 17

Ok, A\* in Rust. Let's see how difficult it is to do. Implementing a simple A* turned out to be a challenge because in a typical implementation, the nodes are static and contain the f and g values and the priority queue entries maintain links to their associated nodes. That causes issues with mutability and lifetimes, the two things that Rust is extremely picky about. In the end, I stored all state about the nodes in the priority queue elements. That was necessary anyway because the pathfinding for the problem is convoluted. Also, the information I can get from static storage is not necessary for solving this problem.

In general, while A\* is the optimal general pathfinding solution, it has implementation issues.

It turns out that there is a way to avoid manipulating the contents of the priority queue. So, not a problem.

| Part | Answer |
|-----:|-------:|
|    1 |    847 |
|    2 |    997 |

## Day 18

Part 1 is pretty straight-forward. Finding a good point-in-poly algorithm proved difficult, though I was able to get away with an adhoc implementation for this problem because I only needed *any* interior point for the flood-fill.

For part 2, I felt that there had to be a simple algorithm for computing the area of this kind of shape, and I was happy to find a simplified variation of the Shoelace Formula, which itself is a simplified variation of Green's Theorem, plus a simple way to calculate the area of a thick perimeter. This solution makes my flood-fill solution for part 1 ridiculous.

| Part |      Answer     |
|-----:|----------------:|
|    1 |           40131 |
|    2 | 104454050898331 |

## Day 19

It is disappointing that the dictionary type in Rust is called `HashMap`. It is almost always better to name something according to what it is or what it does than according to how it is implemented.

| Part |      Answer     |
|-----:|----------------:|
|    1 |          389114 |
|    2 | 125051049836302 |

## Day 20

It seemed fairly straightforward, but Rust's strict mutability and reference lifetime rules made it difficult. I get that Rust forces you to consider potential aliasing and reference problems in order to allow the compiler to make ideal optimization decisions, but for something quick and dirty, I don't mind the compiler assuming worst case.

I removed all the references to eliminate the lifetime headaches. They weren't necessary anyway. I refactored te code to make it a lot cleaner just for fun.

| Part |      Answer     |
|-----:|----------------:|
|    1 |       839775244 |
|    2 | 207787533680413 |

## Day 21

It's a simple depth-first search, however the number of nodes in part 2 is a problem. For part 2, the solution was found by computing the value for step counts of `131 * n + 65`, where `131` is the size of the map and `65` is the remainder of `26501365 / 131`, and then extrapolating that to 26501365 steps.

| Part |      Answer     |
|-----:|----------------:|
|    1 |            3816 |
|    2 | 634549784009844 |

## Day 22

Fairly simple. I had a bug that took a while to find because of all the moving parts. My code requires the bricks to be sorted and I mistakenly assumed that they would remain sorted after falling. It was an easy fix after writing lots of code to check values after every step.

| Part | Answer |
|-----:|-------:|
|    1 |    439 |
|    2 |  43056 |

## Day 23

Pathfinding, but longest path? I don't see how A\* will work. Ended up constructing a directed graph from the map, and then found all possible paths using a depth-first search using recursion. Part 2 was easy as I just switched to a non-directed graph.

As for Rust, I found that I could side-step mutability and lifetime restrictions by storing indexes of vector elements rather than references to those elements. I don't think that bypassing Rust's safety mechanisms like that is a good idea in general. I wonder how many developers do that as a standard practice just for the sake of convenience.

One of the features I am starting to like about Rust is its extensive support for iterators. It is kind of a pain in the ass to have to deal with iterators and Option<> everywhere, but it does allow you to write some concise code.

| Part | Answer |
|-----:|-------:|
|    1 |   2254 |
|    2 |   6394 |

**Note**: Part 2 takes about 3 seconds to run.

## Day 24

Turns out part 1 is just ray intersection, so the only challenge was remembering the math.

| Part | Answer |
|-----:|-------:|
|    1 |  24627 |
|    2 |        |

## Day 25

Well, I learned some graph theory and got more familiar with the intricacies of borrowing and lifetimes. It turns out that a general solution to the problem is intractable. I tried three different approaches before I went with forget the general solution, let's find the answer to this specific problem. The ad hoc approach did the job.

| Part | Answer |
|-----:|-------:|
|    1 | 572000 |
|    2 |    N/A |

**Note**: There is no part 2.
