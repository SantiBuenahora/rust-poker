# REPORT.md
> Santi Buenahora, cis198 final project

## Summary
Working poker program where AI checks by default.

### Week 1
- On week one, I created all the files (`ui.rs`, `main.rs`, `card.rs`, `mod.rs`, `player.rs`, and `table.rs`) in an effort to structure the project.
    - The structure was in part inspired by HW5, the adventure game homework.
- I also wrote the main structs (including `player`, `card`, `hand`, `table`, `hand`) and also the main enums (including `Command`, `Suit`, `HandCategory`).
- A lot of time was also spent creating creation methods for the structs, as well as defining the relationship between the different modules. In other words, how would table be storing the players? I originally had this defined as a `Vec<Rc<RefCell<Player>>>`, and these structural details was gradually modified over time.

### Week 2
- During this week I spent the bulk of my time on `card.rs`, specifically on the `make_hand` function in the `hand` struct. This was surely the most difficult aspect of the project because it required a lot more problem-solving and thinking than the previous aspects.
- I also worked on the ui, which asks players for the number of players as well as their name. This was made possible through the `terminal_request` function, which I later reused in week 3.

### Week 3
- During this week I implemented the core parts of the UI so that a player could give the command that they decided
- I also implemented the `player.act` function, which differs depending on the `is_human` attribute that all players have (see below for more details). This was a difficult part implementation-wise, because my original mentality was to address the Player types using traits to do pseudo-polymorphism.
- The rest of this was finalizing the project so that the final version was playable.

---

## Approximate time spent

- I would estimate I spent like 20-ish hours on this project. I think a big reason for this is that Rust requires a lot of time to plan and design and less time to actually type out the code. There was a lot of writing and then deleting and rewriting. This was especially true in this project than on other homeworks because it was my own design, so there was a lot of tinkering.

---

## Accomplishments
- working UI where the player gives an input among possible choices and the program responds appropriately
- working card comparisions and Hand identification function. This was the part I'm most proud of because I think it required a lot of thought and problem-solving on my end and less busy-work
- pseudo-polymorphism in the design of the player structure, through the traits `ComputerPlayer` and `HumanPlayer`. While this may not seem to valuable, it was a big design decision and somewhere I had to do a fair bit of testing in Rust-Playground. Had I implemented AI, it would have been very difficult to do without repeating code that applies to both players like the attributes and early functions.

---

## Components, structure, design decisions
- I implemented structures for Card, Player, Hand, Table, which made up the core of the project
- For the player implementation I used traits to provide a model for pseudo-polymorphism. The player class implements both traits, which contain all the same functions. I choose which trait to use in the table.rs file, which is very valuable because it allows for less repetitive code.

---

## Testing approach and results
- I tested the UI by trying different inputs and printing to see if the table was changed appropriately.

---

## What you would do differently

- I think a better approach to this project would have been to find an existing poker program (in whatever language) and then fork it and build an AI on top of it. I say this because the AI and the math that would power it was the most interesting thing to me, but I my time building the game logic for poker.

---

## etc.
- Note that while this was built as Texas-Hold-Em, the schema for betting ties back to a simpler version of poker, where the player bets initially and then 
