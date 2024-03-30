# WoRm

This project is an implementation in [Rust](https://www.rust-lang.org/fr) & [Bevy](https://bevyengine.org/) of a worm (or snake) entity which is inspired by Terraria's Worm (more precisely, [The Devourer of Gods](https://calamitymod.wiki.gg/wiki/The_Devourer_of_Gods)).

The worm is a chain of segments that follow the head segment.

![img_1.png](assets/img.png)

Yes, it's a worm. Michael is a worm. Michael is awful and he's a worm.
And now, Michael have a friend: SOLACE OF OBLIVION. Look at him:

![SOO.gif](assets/SOO.gif)

(He is so kawaii ne, like [the devourer of gods](https://www.youtube.com/watch?v=WpPORZMgkFE). Maybe he'll like devouring the devourer of god)

- Michael come from the first (second try) implementation which use [array](#second-approach-worm-and-segments) method.
- SOLACE OF OBLIVION is the second (third try) implementation which use [path](#and-the-fourth-approach-the-most-important) method.

It's my first project in [Rust](https://www.rust-lang.org/fr) and [Bevy](https://bevyengine.org/), so be indulgent. I'm open to any advice or suggestion.
I've voluntarily leaved some comments (useful and useless).

## How to run the project?

### Prerequisites

- [Rust](https://www.rust-lang.org/fr)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Run the project

```
cargo run
```

## How do I've implemented the worm?

### First approach: linked list

In the file [worm_linked_list.rs](src/worm_linked_list.rs), I've tried to implement the worm as a linked list. Each segment has a reference to the next segment.

```
// Head is just a Tail with a Tag 
Tail -> Option<Tail>
```

(It doesn't work at all)

### Second approach: array

In the file [worm_array.rs](src/worm_array.rs), I've implemented the worm as a struct with a Vec of segments. I get the Worm struct and iterate over the segments to update their positions in correct order.

```
Worm -> Vec<Entity>
```

### Maybe a third approach?

I'm thinking about a third approach where the head and tails are separated. The head will emit an event when it moves and the tails will listen to this event to update their position. I don't know if it's a good idea.

### And the fourth approach (the most important one)

The problem with the previous methods is that the update of the parts is not optimised in the context of an E.C.S. architecture: the update of the parts depends on the previous parts, and so we are obliged to update the parts of the worm all at once, in a loop.

So we have to update the parts in no particular order. This is quite a complex challenge, at least to obtain a result similar to the [Array](#second-approach-worm-and-segments) method.

To solve this, I thought of creating a path on which the parts could be updated without taking into account the position of the other parts. This path would be included in a "Worm" component. This "Worm" would then have children, each with an index allowing a reference to be obtained in the path of the worm.

You can look at [worm_path.rs](src/worm_path.rs) for more details.

```
Worm -> Path (List of points (position), number max of points, and length of path)

Worm <-Parent/Child-> WormPart

WormPart -> Id
```

## Inspired by

For Array:
- https://github.com/bevyengine/bevy/issues/9228
- https://mbuffett.com/posts/bevy-snake-tutorial/

For Path:
- https://www.reddit.com/r/gamemaker/comments/5f02q3/how_can_i_make_a_worm_enemy_in_my_game/
