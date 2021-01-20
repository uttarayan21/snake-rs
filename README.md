## Snake in rust using [ncurses](https://docs.rs/ncurses)

The game is playable

run with

```bash
cargo run
```

#### Keybinds

hjkl or arrow keys for movement

esc or p to pause

#### Screenshot

Click the image for asciinema

[<img src="images/screenshot.png" width="650" />](https://asciinema.org/a/PtMG7dghPAEZ7tNgx70sKplKq?autoplay=1)

#### Todo :construction:

- [ ] Add a way to change the snake speed
  - [ ] Make a ui to switch the snake speed
  - [x] Interal implementation of the snake speed
- [ ] Implement Highscore System
  - [ ] Make the ui
  - [ ] Internal Implementation

#### Bugs :bug:

- ~~Snake going through the walls~~
- ~~Food spawning in the walls~~
- Remove all the logging in the ui
- Pausing delayed if esc is pressed but not if p is pressed.

#### Maybe in the future

- [ ] Autoplay the game using a simple pathfinding algorithm (to show on [r/unixporn](https://reddit.com/r/unixporn) of course :clown_face:)

#### Notes

A few notes about the complexity of the game and how I should improve the game

<details>
<summary>Read More</summary>

The complexity of the program is O(n) every tick (time which changes relative to the speed)

However the place, I can improve is the redrawing of the game

As of commit
<a href="https://github.com/uttarayan21/snake/commit/de66f7d249a56f883dd632598a4178b1bd1320ba">f9be68e</a>
the game redraws the total board and the total snake every tick.

I think this can be improved by only drawing the parts of the snake and the board when needed

</details>
