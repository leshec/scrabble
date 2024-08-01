use rand::Rng;
use std::collections::HashMap;
use std::fmt;

static DATA: &'static str = include_str!("data");
const NUMBER_OF_TILES: u8 = 7; //number of tiles fixed here

fn main() {
    //quick dev tests
    //assert_eq!(score_word("guardian".to_string()), 10);
    //let mut player_tiles = vec!['d', 'e', 'c', 'i', 'm', 'a', 'l'];
    //player_tiles.sort();

    let player_tiles = make_a_set_of_random_tiles();
    println!("The player tiles are {:?}", &player_tiles);
    let subsets = produce_tile_subsets(player_tiles);

    let dictionary = make_dictionary(&DATA);
    let mut results_list: Vec<String> = make_results_list(dictionary, subsets);

    if !results_list.is_empty() {
        let answer = build(&mut results_list);
        println!("{}", answer);
    } else {
        println!("Sorry no results for those letters.");
    }
}

#[derive(Debug)]
struct Answer {
    valid_words: Vec<String>,
    longest_valid_words: Vec<String>,
    highest_scoring_words: Vec<String>,
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "highest scoring words: {:#?} \nlongest scoring words {:#?} \nvalid_words {:#?}",
            self.highest_scoring_words, self.longest_valid_words, self.valid_words
        )
    }
}

fn build(results_list: &mut Vec<String>) -> Answer {
    if results_list.is_empty() {
        panic!("Results list is empty and this function shouldn't be called")
    };

    results_list.sort_by(|a, b| (a.len()).cmp(&b.len()));
    results_list.reverse();

    let mut longest_words: Vec<String> = Vec::new();
    let mut highest_words: Vec<String> = Vec::new();

    let longest_word_length = &results_list[0].len();

    for item in results_list.iter() {
        if item.len() < *longest_word_length {
            continue;
        } else {
            longest_words.push(item.to_string());
        }
    }

    for item in longest_words.iter() {
        let score = score_word(item.to_string());
        highest_words.push(format!("{} = {}", item, score));
    }

    Answer {
        valid_words: results_list.to_vec(),
        longest_valid_words: longest_words,
        highest_scoring_words: highest_words,
    }
}

fn make_a_set_of_random_tiles() -> Vec<char> {
    let mut bag = Vec::new();
    const ASCIISET: &[u8] =
        b"aaaaaaaaaiiiiiiiiioooooooonnnnnnrrrrrrttttttllllssssuuuuddddgggbbccmmppffhhvvwwyykjxqz";
    for _ in 1..(NUMBER_OF_TILES + 1) {
        let idx = rand::thread_rng().gen_range(0..ASCIISET.len());
        bag.push(ASCIISET[idx] as char)
    }
    bag.sort_by(|a, b| a.cmp(b));
    return bag;
}

fn score_word(word: String) -> usize {
    let mut score: usize = 0;

    for letter in word.chars() {
        match letter {
            'e' | 'a' | 'i' | 'o' | 'n' | 'r' | 't' | 'l' | 's' | 'u' => score += 1,
            'd' | 'g' => score += 2,
            'b' | 'c' | 'm' | 'p' => score += 3,
            'f' | 'h' | 'v' | 'w' | 'y' => score += 4,
            'k' => score += 5,
            'j' | 'x' => score += 8,
            'q' | 'z' => score += 10,
            _ => println!("missed a case scoring letters"),
        }
    }
    return score;
}

fn produce_tile_subsets(player_tiles: Vec<char>) -> Vec<String> {
    let mut subsets: Vec<String> = Vec::new();

    for n in 0..1 << NUMBER_OF_TILES {
        let mut subset = String::new();
        for (idx, tile) in player_tiles.iter().enumerate() {
            if n & (1 << idx) != 0 {
                subset.push(*tile);
            }
        }
        if !subset.is_empty() {
            subsets.push(subset);
        }
    }

    subsets.sort();
    subsets
}

fn make_dictionary(lines: &'static str) -> HashMap<String, String> {
    let mut dictionary: HashMap<String, String> = HashMap::new();

    for line in lines.split("\n") {
        if line.trim().len() <= (NUMBER_OF_TILES).into() {
            let mut word: Vec<char> = line.trim_end().chars().collect();
            word.sort();
            let alpabeticised_word: String = word.into_iter().collect();
            dictionary.insert(line.trim().to_string(), alpabeticised_word.to_string());
        }
    }
    return dictionary;
}

fn make_results_list(dictionary: HashMap<String, String>, subsets: Vec<String>) -> Vec<String> {
    let mut results_list: Vec<String> = Vec::new();
    for (key, value) in dictionary {
        if subsets.contains(&value.to_string()) {
            results_list.push(key);
        }
    }
    results_list
}

// fn tests() {
//     assert_eq!(
//         produce_tile_subsets(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
//         vec![
//             "a", "ab", "abc", "abcd", "abcde", "abcdef", "abcdefg", "abcdeg", "abcdf", "abcdfg",
//             "abcdg", "abce", "abcef", "abcefg", "abceg", "abcf", "abcfg", "abcg", "abd", "abde",
//             "abdef", "abdefg", "abdeg", "abdf", "abdfg", "abdg", "abe", "abef", "abefg", "abeg",
//             "abf", "abfg", "abg", "ac", "acd", "acde", "acdef", "acdefg", "acdeg", "acdf", "acdfg",
//             "acdg", "ace", "acef", "acefg", "aceg", "acf", "acfg", "acg", "ad", "ade", "adef",
//             "adefg", "adeg", "adf", "adfg", "adg", "ae", "aef", "aefg", "aeg", "af", "afg", "ag",
//             "b", "bc", "bcd", "bcde", "bcdef", "bcdefg", "bcdeg", "bcdf", "bcdfg", "bcdg", "bce",
//             "bcef", "bcefg", "bceg", "bcf", "bcfg", "bcg", "bd", "bde", "bdef", "bdefg", "bdeg",
//             "bdf", "bdfg", "bdg", "be", "bef", "befg", "beg", "bf", "bfg", "bg", "c", "cd", "cde",
//             "cdef", "cdefg", "cdeg", "cdf", "cdfg", "cdg", "ce", "cef", "cefg", "ceg", "cf", "cfg",
//             "cg", "d", "de", "def", "defg", "deg", "df", "dfg", "dg", "e", "ef", "efg", "eg", "f",
//             "fg", "g"
//         ]
//     );
// }
