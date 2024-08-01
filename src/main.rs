use rand::Rng;
use std::collections::HashMap;

//
// Calculate the score for a word. The score is the sum of the points for the letters that make up a word. For example: GUARDIAN = 2 + 1 + 1 + 1 + 2 + 1 + 1 + 1 = 10.
// Assign seven tiles chosen randomly from the English alphabet to a player's rack.
// In the real game, tiles are taken at random from a 'bag' containing a fixed number of each tile. Assign seven tiles to a rack using a bag containing the above distribution.
// Find a valid word formed from the seven tiles. A list of valid words can be found in dictionary.txt.
// Find the longest valid word that can be formed from the seven tiles.
// Find the highest scoring word that can be formed.
// Find the highest scoring word if any one of the letters can score triple.
// For discussion: how would we adapt our solution for a multiplayer environment?

//number of tiles fixed here
const NUMBER_OF_TILES: u8 = 7;

static DATA: &'static str = include_str!("data");
fn main() {
    assert_eq!(score_word("guardian".to_string()), 10);

    let player_tiles = make_a_set_of_random_tiles();
    //let player_tiles = vec!['a', 'a', 'd', 'g', 'i', 'n', 'r', 'u'];
    println!("The player tiles are {:?}", &player_tiles);
    let subsets = produce_tile_subsets(player_tiles);
    //do this properly
    //    tests();

    let dictionary = make_dictionary(&DATA);
    let results_list: Vec<String> = make_results_list(dictionary, subsets);

    let mut final_results_list = results_list.iter();

    while let Some(result) = final_results_list.next() {
        println!("{:?}", &result);
    }

    get_answers(results_list);
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
            _ => println!("missed a case"),
        }
    }
    return score;
}

fn produce_tile_subsets(player_tiles: Vec<char>) -> Vec<String> {
    //let number_tiles: u8 = player_tiles.len().try_into().unwrap();

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

fn get_answers(results_list: Vec<String>) {
    let mut answer: Vec<(usize, &str)> = Vec::new();
    for result in results_list.iter() {
        if !results_list.is_empty() {
            answer.push((score_word(result.to_string()), result));
        } else {
            println!("Results list is empty");
            answer.push((0, "0"));
        }
    }
    answer.sort();
    if answer.is_empty() {
        println!("There is no scoring word");
    } else {
        println!("Top scoring word {:?}", answer.pop().unwrap());
    }
}

