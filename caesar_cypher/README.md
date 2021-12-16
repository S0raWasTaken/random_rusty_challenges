# CAESAR_CYPHER
An "inferi" challenge
Targets:
- Small code
- Functional

Did I exagerate a little bit?: Yes

## Steps
### Compiling
> I'll assume you have rust installed in your system
> If not, check out [rustup](https://rustup.rs)
```bash
git clone https://github.com/S0raWasTaken/random_rusty_challenges.git
cd random_rusty_challenges/caesar_cypher
cargo build --release
mv target/release/caesar_cypher .
```
### Using
> Default value for rotation is 13
Simple usage:
```bash
./caesar_cypher abcdefghijklmnopqrstuvwxyz
```

Specify rotation:
```bash
./caesar_cypher abcdef -r 15
```

Why did I consider it exagerated?
1. 70 lines
2. Handles all kind of errors and user stupidness
3. Has `--help`
4. Also helps the user if it misses something or send wrong values to the options 
