# NO_STD_MAP
Actual crate name is soramap, but please ignore it...

## Using
If that's useful, here's how you'll add it to your project
### Adding to your projects folder
```bash
# I will assume that you are already inside your projects folder
mkdir rusty_challenges
cd rusty_challenges
git init
git remote add origin https://github.com/S0raWasTaken/random_rusty_challenges.git
git fetch origin
git checkout origin/master -- nostdmap
```
The folder will be in `rusty_challenges/nostdmap`.
### Adding to Cargo.toml
Add this to the end of your `Cargo.toml`:
```toml
[dependencies.soramap]
path = "/path/to/rusty_challenges/nostdmap"
version = "*"
```
Don't forget to change `/path/to` to your actual path to the lib<br>
Example: `../rusty_challenges/nostdmap`

### Actually using in your code
It should be straight forward, I only implemented 2 methods and 2 constructors for `SoraMap<K, V>`
```rs
use soramap::soramap::SoraMap;

fn main() {
	// Building a new map:
	let mut map: SoraMap<i32, &'static str> = SoraMap::new(); // could also be S0raMap::default();

	// Setting a value:
	map.set(30, "Cool number").unwrap(); // Returns a Result<(), &'static str> since I'm too lazy to make a struct for errors
	map.set(69, "Nice number").unwrap();

	// Getting a value:
	let val = map.get(69).unwrap(); // Returns Option<V>

	// Print it, so that you know it works
	println!("{}", val);

	// And why not printing the whole map?
	println!("{:?}", map); 
}
```

### Final nerdy but dumb explanation
Basically, key and value are both 2 different maps, each with the same size. <br>
It makes it so when you search for Key in the vector, you end up getting the Value's index in the other vector.<br>
Also, key is a `PartialEq`, meaning that I can actually try to match stuff as `k.eq(key)`
