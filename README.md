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

[![snake](images/screenshot.png =650x)](https://asciinema.org/a/PtMG7dghPAEZ7tNgx70sKplKq?autoplay=1)

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
