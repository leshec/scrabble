# scrabble

**Toy project**

**Scrabble challenge from Guardian newspaper coding challenges.**
I'm trying to learn rust...
[guardian-coding-challenges](https://github.com/guardian/coding-exercises/tree/main/scrabble)

Example ouput: 

The player tiles are ['d', 'j', 'k', 'l', 'o', 'r', 'r']
highest scoring words: [
    "dork = 9",
    "lord = 5",
    "dorr = 5",
]

 longest scoring words [
    "dork",
    "lord",
    "dorr",
]

 valid_words [
    "dork",
    "lord",
    "dorr",
    "old",
    "dol",
    "dor",
    "kor",
    "rod",
    "or",
    "jo",
    "lo",
    "do",
    "od",
]

**Todo:**
1) tidy up and rename stuff
2) make formatting prettier
3) add tests and error handling
4) fix stuff the test shows fails
5) add the triple letter feature:
Find the highest scoring word if any one of the letters can score triple.
6) Maybe modularise the functions into a lib.rs project structure

**Extension ideas**
1) make a basic CLI app using clap that I could use to beat friends/computer at Scrabble:
with a help flag to make a menu, defaults, user selects letter, chooses the number of tiles
2) try to implement with a Trie data structure
3) try to implment using a recursive match call:
an idea I had but wasn't sure how to implement that may make it faster
4) get a basic web page showing results using HTMX
5) try to populate and search an Sqlite or Postgres db
6) make the actual scrabble game...
 
 **install from terminal**
```bash
git clone git@github.com:leshec/scrabble.git
cargo run
```

