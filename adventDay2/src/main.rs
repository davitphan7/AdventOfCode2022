
use std::{collections::HashMap, io::Split};
fn main() {
    let s =
"B Z
C Z
C Z
A Y
B Z
C Y
C Z
C Y
C X
A Z
B Z
C Z
A Y
C Z
B X
C Y
C Z
C X
C X
C Y
C Y
C X
A Y
C Y
C Y
C X
C Z
C X
A X
C Z
C Y
C Z
C Z
C Z
A Z
C Z
B Z
A Y
A X
A Y
C Z
A Y
C Z
A Z
C Y
C Y
C Y
A X
C X
C X
C Y
A X
A Y
C X
A X
C Z
A Y
C Z
C Y
A Y
C Z
A X
A Y
A X
C X
C Z
A X
B Z
C Z
C Y
C X
C X
C Z
C X
C Z
A X
C Y
A Y
C Y
C Z
C X
B X
B Z
C Z
C X
A X
B Z
A Y
A Y
B Z
A Y
C Y
B Z
B X
C X
C X
C X
A X
A Z
A Y
A Y
C Y
C Z
C Z
C Z
A Y
C X
A Y
A X
C X
C X
A X
A Y
A X
A X
A Y
C Y
A Y
A Y
A X
C X
B Z
A X
C Y
C X
C Z
C Z
C Z
C Z
C Y
C X
C X
A X
A Y
A X
C Z
C X
C Y
C Y
C Z
C Z
B Z
C Z
C Y
C X
C Y
A X
A Y
C X
B Z
B Y
C Z
C Y
C X
A Y
C Y
B Z
A Z
C X
C Z
C Y
C X
C Z
A X
A Y
C Y
C Y
A Y
A X
A X
C X
C X
C Z
A Y
A Y
C Y
C Z
A X
A Y
C X
C X
C Y
C X
C Z
C X
C Z
C Z
C Z
C Z
A Y
B Z
C Y
C Z
A Z
C X
C X
A X
A Z
C Z
C X
C Z
A X
B Z
B Z
A X
C X
C X
B Z
B Z
B Z
C Z
C X
A Y
C Z
A X
C Y
C Z
C Z
C X
C X
C Z
C Y
C Y
C X
C X
A X
C Y
A X
C Y
C X
B X
B Z
C X
C X
B X
C Z
C Y
A X
C Z
C Z
C Z
C Y
C X
A Y
A Z
C Z
B Z
C Z
C X
A Y
C X
C X
A X
C X
C Z
A X
C Z
C Y
C Y
C Y
C Z
A X
C Y
C X
C Z
A X
C Z
C X
C X
C X
A Y
C Z
A Y
C Z
C Y
C Z
A Y
C X
C X
C X
C Y
C Z
C X
C Y
C X
A X
C X
C Y
C X
C Z
C X
B Z
C X
B Z
B X
C X
C X
C X
B X
A Y
A Y
A X
B Z
C X
A Z
C Y
C X
C X
C Y
A Y
A Z
C X
A Y
C Z
C X
C X
C Y
C Y
A Z
A X
C Z
C X
C Z
B Y
A Y
C Z
A Y
C Z
A X
C Y
C Y
C X
A Y
C X
C X
B X
C Z
C X
C Y
C Z
C Z
C Y
C Y
C Y
C X
B Z
C X
C X
C Y
C Y
C Z
B Z
C X
C Y
B Z
C Z
A Y
C X
C X
A Y
C Y
C Z
C X
A Y
A Y
C Z
C Z
C Z
A X
C X
C Z
C Z
C Z
A Y
A X
C X
C Z
C X
C X
C Y
A Y
C X
C Y
C Z
C Y
A X
C X
C X
C Z
A X
B X
C Y
A X
C Y
C Y
C X
C Z
B X
C X
C Z
A Y
C Z
C X
A X
C Y
A X
C Z
A X
C X
C Y
C X
C Z
C X
C X
C Y
A X
C Y
A Y
C Z
C X
C X
A X
A Z
C X
C X
C Y
B Z
A X
A X
C X
B Z
C Z
A Y
C Y
C Z
C X
A Z
C Y
A X
C Y
C Z
C Z
C X
C Y
C X
C Y
C Y
C X
A Y
B X
C X
A X
A Y
C Y
C Z
C Y
C X
A Y
C Z
C X
C X
A Z
C Y
C Z
A X
C X
C Z
A Y
A X
B X
C Z
B Z
C X
A Z
C Z
A X
A Y
C X
A X
B Z
C X
C Y
C Z
C Y
C Y
C Z
C X
B X
C Y
B Y
C Z
A Y
A Y
C Y
C Z
A X
C Z
C Y
B Z
C X
A X
C X
C X
C Y
C Y
C Z
A Y
C Z
A X
A Y
C X
C Y
C Z
C X
A X
B Z
C Z
A X
C X
C X
C X
B Z
C Z
C Y
B Z
A X
C Z
C Y
A X
C X
B Z
C Y
C X
C Z
C Z
C Z
C X
C Y
B X
A X
B Z
C X
C Y
C Z
C X
C Z
C Z
C Z
C Y
A X
C X
C Z
A X
C X
C Z
B X
C Z
B Z
A Y
A X
C Z
C Z
C Y
C Z
C Y
C Z
A Y
B Z
A Y
C X
A X
A X
A X
B Z
C Z
B Y
C Y
A Y
A Y
C Z
A Y
C Z
C Z
A Y
C Z
C Z
C Z
A Y
C Z
C Z
C Z
C Z
A Y
C Y
A Y
C Y
C X
C X
C Z
C X
C Z
C Y
A X
C Z
B X
C Z
A X
C Z
C Z
C Y
C Z
C Y
C X
C Y
B X
C Z
C X
C Y
A Y
C X
C Z
C X
C Z
C X
A Z
C Z
C X
C X
B Z
C Z
C X
A X
C Y
A Y
C X
C Z
C Z
A Y
A X
B Y
C Y
C Y
C Y
C X
A X
C Z
C Y
C Z
C X
C Z
C Z
C Y
B X
C X
C Z
C X
C X
C Y
A X
B Z
A X
C X
C Z
B Z
A Y
C Y
B Z
C X
A Z
A X
C Z
C Z
C X
A Y
B Y
C X
C Z
B Y
A Y
A Y
C X
A X
C Y
C X
C X
C Z
A X
A X
A X
B Z
C Y
C Z
C X
A X
B Z
C Z
C Z
C X
C X
C X
C X
C Z
C Y
C Z
C Z
C Z
A Y
C Z
C X
C Y
C X
B Y
A Y
C Z
C X
C Z
A X
A X
C Y
C Y
C X
C Z
C Z
C Z
C X
C Z
C X
C Y
C X
C X
C Y
C Y
B Z
C Z
C Z
B Z
C Y
C Z
C Y
B Z
C X
C Z
C Y
C Z
A Y
C Z
B Z
C Z
C X
C X
A Z
C Z
C X
A Y
A Z
A X
C Y
C X
C Z
C X
C X
C Y
C X
C Z
C X
C Y
C Y
C Z
C Y
C Y
C Y
A Z
A Y
C Z
C Y
C Z
C X
A X
C Z
C Z
A Y
C Z
C X
C X
C X
C Z
C X
C Z
C Y
A Z
C Z
C Y
C Z
C X
C Y
C Y
C X
C X
C Z
A Y
C Y
B Z
A Y
C Y
C X
C Z
C X
C Y
C Z
A Y
A X
C Z
B Y
C Y
A X
C Y
C Z
B Z
C Y
C Z
C Z
B Z
C X
A Y
C Z
C X
B Y
B Z
C Z
B Z
C X
A X
A X
B Z
C Z
C Z
C Z
C Y
A Y
C Z
B X
C Y
A Z
C Y
C Y
A Y
C X
C Y
B Z
C X
C X
C X
B Z
C X
B Z
A X
C Y
C Y
A X
C X
A Y
C X
A X
C Z
A X
C X
C X
A Y
C Z
C Z
C X
C X
B Z
A Y
C Y
C X
A Y
C Z
A X
C X
B Z
C X
C X
B X
B X
B Z
C Z
C X
A Y
B Z
B Y
C X
C Z
A Y
C Z
C X
C X
B Z
C X
A Y
A X
A Z
C Z
C Y
B Z
C X
A X
C X
A X
C X
C X
C Z
A X
B Z
C Z
C X
C X
A X
C Z
C X
C X
C Z
C Y
C X
A X
C X
C X
C X
A X
C Z
C Z
C Y
B Z
C X
A Y
C Z
C X
C Y
B X
A X
C X
B Z
C Y
B X
A Y
B X
C Z
C Y
C X
C Z
A Y
A Y
C X
A X
C X
C X
A Y
A X
B X
C Y
A Y
C X
C Z
A Y
C Z
B Z
C Z
A X
C Y
C Y
C Y
A Y
A Z
A Y
C X
C X
A Y
C Y
A X
C Y
A Y
C X
C Y
B Z
C Z
C Z
B X
C Y
B X
A Y
A Y
A X
A Y
C Z
C Z
C Y
B Z
A Z
C X
C Z
A X
C X
C X
C Z
C Y
C X
A Y
C Z
B Z
A Y
A X
C Y
B X
C Z
A Y
C X
C Y
C X
A X
C Z
C Z
C X
C Z
A X
C X
C X
A Z
B Z
A X
C Y
C X
A X
C X
C Z
C Y
A Y
C X
A Y
A X
C Y
A Y
C X
A Y
C X
B Z
C X
C X
C X
C Z
C Z
C Z
B Z
C X
B Z
C X
A X
A Y
A Y
A X
C Y
A X
C Y
A Y
B Z
C X
C X
C Y
C X
C Z
A X
A Y
C Z
A X
C X
C X
B X
C X
A Y
C X
C Z
B Z
A X
B Z
C Y
C Z
C Y
C Y
C X
C Z
C Z
C Z
A X
C X
A X
A Y
A X
C Z
C Z
A Z
C Y
B Z
A X
C Z
C Z
C X
C X
A X
A Z
C Z
C X
C X
C Y
C Y
C Y
A Y
A Y
C Z
A X
C Z
C X
A X
C Y
C Z
C Y
A X
A Y
A X
C Z
B Z
C Z
A X
C X
C Y
C X
C X
C Y
C Z
C Z
C Y
C X
A Y
C Y
C Z
B Z
C Y
C Y
A X
C Z
B Z
C X
B Z
B Z
C Z
C X
C Z
A X
C Z
C X
A Y
A X
A Y
C Z
C Z
C Z
C Z
C X
B Z
C Y
C X
B Z
C X
A X
A Y
C X
A Z
A Y
A X
C Y
C Z
C Y
C Z
B Z
C Y
C X
A Y
B Z
C Z
C Z
C Y
C X
C Z
A Y
C Y
A Y
C Z
B Z
C Z
A X
C Z
C Z
C Y
C X
C Y
A X
C Y
C Z
C X
A X
B Z
C Y
C X
C Z
C Y
A X
C X
A Y
A Y
C X
C Z
C Y
B Z
B X
C Y
C X
C X
C Z
C Y
C X
C Z
C Y
A X
B Y
A X
A Y
C X
A X
A X
A X
C Y
B X
B Z
C Z
C X
C Z
A Y
C Y
C X
B Z
A X
C X
A Y
C X
C X
C X
B Z
C Z
C X
B X
C Z
C Y
C X
C Y
C Z
C Z
C Z
A X
C X
C X
C X
B Z
C Z
A Y
C X
B Z
B Z
A Y
A Y
B Z
C X
C Y
A X
C Y
C X
C Y
C Y
C X
B Z
C Z
C Z
C Z
C Y
A Y
C Z
C Y
A X
C X
C Z
C Y
A Y
C Z
C X
B Z
C X
C Z
C Z
C Y
A X
A Y
C Z
A Y
C Z
C X
C Z
C Z
A X
A X
C X
B Z
B Z
C Y
A Y
C X
C X
C Z
C Z
A Y
C Y
C Z
A X
C X
C Y
A Y
C X
C X
A Z
C X
C Y
C Z
C X
B Z
C X
A X
C Z
C Z
A X
C Z
C Y
C Z
C Z
A Y
C X
C X
C X
C Y
C Z
C X
C X
C Y
B Z
A X
C X
A Y
B Z
A Y
A X
A X
B X
C Y
A Y
C Y
C Z
A X
C X
A Y
A Y
C Z
C X
C Z
C Z
C Z
C Z
A Y
A Y
A X
C X
A Y
C Y
A X
C X
C X
C X
B Z
C Z
B Z
A X
C Z
C Z
C Y
C X
C X
C X
C Y
A Z
B X
B Z
C X
B Z
A X
A Y
C Z
B Y
C X
C X
A Y
C Y
C Y
C Z
C X
A X
B Z
C X
A X
A X
C X
A X
C Y
B X
C Z
C Z
C Z
C Z
C X
C Z
C X
C Z
C Z
A X
C Z
C X
B Z
A Y
C Z
B Z
C Y
C X
C X
C Z
C Y
C X
C Z
C X
B X
C Z
A Y
C X
B Z
A Y
A X
C X
C X
B Z
C Y
C Z
C X
C Z
C Z
C Z
B X
C X
C X
C Y
B Z
A Y
C Y
C Z
A Z
A Y
C Z
C Y
C Y
A X
B Z
B Z
C X
A X
C Z
B Z
C Z
C Y
C Z
C Y
A X
C X
C Y
C X
B X
C Y
A Y
C X
C Z
A Z
C Y
A X
C Z
C Y
B Z
C Y
C Z
A Y
C Z
C X
A Y
C X
C Z
C X
B Z
C X
A X
C Z
C Z
C X
C Z
C X
C Z
C Z
A Z
A X
B Z
B X
C Z
C X
B Z
C X
C Y
C Z
C Z
A X
A X
C Y
B Z
A X
C X
C Z
A Z
A Z
C Z
C Y
C X
C Y
C Z
A Z
A X
A Y
C X
C Z
A Y
C Y
A Y
A X
C Z
C X
C Z
C X
B Z
A Y
C X
A X
C X
B X
C X
A X
C X
A X
C Y
A Y
C X
A Y
C X
C Z
C X
B Z
A X
C Z
A X
C Z
C Z
C Y
C Z
C Y
C X
A Y
C X
A X
C Z
C X
A Y
C Y
C X
C X
C Z
C X
C Y
C X
C X
C X
C X
A Y
A Y
A Y
C Z
C X
B Y
B Y
A Y
C Z
C X
A Y
C Z
C Y
C X
C Z
A X
C Z
C Z
C Y
A X
A X
C Z
C X
C X
C X
C Z
C Z
A Y
C Y
A X
B Z
C Z
C Y
C Z
A X
C Z
C X
C X
A Y
C Z
C X
C Z
C Z
A Y
C Y
C Z
A X
C Y
C Z
A X
C X
A X
C X
C Z
C X
C Z
C Z
C Y
C Y
A Y
C Y
A X
C Y
C X
C Z
C X
A X
C X
A X
A Y
A X
C Z
A X
C Y
C X
A Y
C Z
C Z
C X
B X
C Z
C Y
A Y
B Z
C Y
C X
B Z
C X
A Y
A X
A X
A X
C X
A X
C X
C Z
C X
A X
C Y
A Y
C X
C Z
C X
C Z
C Y
C X
A X
C X
C Y
A Y
A X
B Z
C Y
B Z
C Y
C X
A Y
C X
C Z
C X
C Y
A X
C Y
C Y
C Z
C Y
C X
C Z
C X
C X
C X
C Z
C X
A Y
A Y
C X
C Z
B Z
C X
C Y
C X
C X
C Z
C Z
C X
C X
A Z
A X
C X
B Z
C Z
C Y
C Z
C X
C X
C X
C Z
C X
A X
A X
C Z
C X
A Z
C X
A X
A X
C Z
A X
A Y
C Z
A Z
A Y
C Y
C X
A Y
A X
C Z
C X
A Y
B Z
C Z
A X
C Z
C X
A X
A Y
C Z
C Y
C Z
C Z
B Z
C X
C Z
C Z
C Y
C X
C Z
C X
C X
B Z
B Z
A Y
B X
B Z
C Z
A Z
C Y
C X
C Z
C X
C X
A X
C X
A Y
B X
C Z
C Y
C X
C Z
C Y
C Y
C X
B Z
C X
B Z
C Y
C Y
C Y
C X
C X
A X
A X
A X
C Z
A X
C Z
C X
C Y
C Z
C Y
A Y
C Y
A X
A Y
C Y
C Z
C Z
A Y
C X
C X
A X
C Y
C X
C Y
A X
C X
C Y
C Z
C Z
B Z
A Y
C X
B Z
C Y
A X
C Z
C X
C Z
B X
C Z
C Z
C Z
C X
C Z
C Z
C X
C Z
C Z
B X
A X
C Y
C Z
A Y
C Y
C Z
C Z
C X
A Y
A X
C Y
C Y
A Y
C Z
C X
C X
C X
A Y
C X
B X
C Z
C Z
C X
C Z
C X
A Y
A Z
C Z
C Z
B X
C Y
A Z
A Y
C Z
C X
C Z
C X
A X
A Y
C Y
A X
C Z
C X
C Z
B X
C X
C Z
B Z
C Z
C Y
C Y
A X
C Z
C X
C Y
C Z
C Z
A Y
C Z
C Y
C Y
A X
B Z
C Z
A X
C Y
C Y
C Z
A X
C X
C Z
C X
C X
C Z
C Y
C X
C Z
B Z
C Y
A X
A X
C X
C Z
A X
C X
B Z
C Y
C Z
C Y
C X
C Z
B Y
C X
C Z
A X
C X
A X
C X
C Z
B Z
C Z
C Z
A Z
C Y
C X
A X
C Z
A X
A Y
C X
C Z
C Y
A Y
C Y
C X
A X
C X
C Y
C X
C X
C Z
C Z
C X
C Z
A X
A X
C X
C Z
B Z
A X
C X
C Z
C Y
C X
B Z
C X
C X
C X
B X
C Y
A Y
C Z
A Z
A Y
A Y
C Y
B Z
C Z
C Y
C X
C Z
C X
B Z
B X
A X
A Z
A Y
C Z
C Y
C Y
C X
B Y
C Y
C X
C X
B X
C Y
C Z
B Z
C X
C Y
A Y
C Z
C X
C Z
A X
C Z
A X
A Y
C Y
A Y
C Y
C Z
A X
C Y
C Y
C X
B X
C Z
C X
C Z
C Z
C Z
C X
C Z
C Y
C X
C Y
C X
C X
C X
C X
A Y
C X
A Y
C X
C Z
A X
A Z
C Z
C X
C X
C X
C X
B Z
C Z
C Z
C Y
C X
C Z
C X
B Z
B Z
C Z
C Z
C X
C X
C Z
B Z
C X
A Y
C X
C Z
C X
C Y
C Z
C Z
C X
C Z
A Y
A Y
C X
C Z
C Z
C X
C X
B Z
C X
C Z
C Z
C X
C Z
A X
C Z
B Z
C Z
A Y
C Z
C X
C X
B Z
A Y
C Z
A Y
C Y
A Z
C X
A X
C X
A Y
C Z
C X
C Z
C Z
C X
C Z
A X
B Z
C Y
C Z
C Z
C X
C Z
B Z
C Y
C Z
C Z
A X
B Z
A X
B Z
C X
C Z
A Z
A Z
C X
A X
A X
B Z
A Y
A Y
A Z
C X
C Z
C Z
C X
C X
C X
A X
C X
C Z
C Y
C Z
C X
C Z
C X
A X
A Y
C X
A X
C X
B Z
B Z
B X
C X
A X
B X
C X
A Y
C Z
C Y
C Z
C X
A X
A Y
C Z
B Z
C X
B Z
C Z
B Z
A X
A X
C Y
B Z
C X
B Z
A Y
C Z
A Y
C Z
C X
A Z
C Z
C Z
B Z
A Y
B Z
C Y
C X
A Y
A X
A X
C X
C Y
B Z
C X
C Z
C Z
A X
A Y
C X
C Y
B Z
C Z
C X
A X
C Z
C X
C X
B Z
C Y
A Y
A X
C Z
B Z
C X
C Z
A Y
C X
B Z
C Y
B X
C X
C X
C Y
A Y
A X
C Y
C Y
C Z
C Y
C Y
C X
B X
C X
A Y
C Z
C Z
A Y
C Z
C Z
C Z
C Z
C Z
C Y
A Y
C Z
A X
C Y
C Z
C X
C Z
C X
A Z
C X
A Y
B Y
B Z
A Y
A Y
C Z
C X
B Z
B X
B Y
C X
C Z
B Z
A X
C X
C X
C Y
A X
C Z
C Y
A Y
A X
A X
B X
A Y
C Y
C Z
C Z
C X
C X
A X
C Z
A Z
A Y
A X
C X
A Y
B Z
C X
C Z
C Y
C Z
C Z
C X
C Z
A X
A Y
C X
C X
C X
C Z
C Y
C Z
B Z
C Z
B Z
A Y
A X
C Z
C X
C Z
C X
A X
C Z
C Z
C X
C X
C Z
A Y
A Z
A Y
C Z
C Z
C X
C Y
C X
C Y
A X
C X
A Y
B Z
B Z
C X
C X
C Z
C X
A Y
A Z
A Y
C X
C Z
A Y
C Y
C Z
B Z
A X
C X
C Z
C Z
A Y
A Y
A Y
A Y
C X
B Z
A Z
B X
A X
B X
C Z
C Z
C Z
B Z
A Y
C Z
C Y
C X
C Y
A Y
C X
C Y
C Y
A X
C Y
C X
C X
C X
C X
A Y
A Y
C X
C Z
C Z
A Y
C Z
A Y
C Z
C Z
C X
C X
C Z
C Y
C Z
A X
A X
C Y
B Z
A X
B Z
B Z
C X
C Z
C Y
A X
C Z
C Y
C Z
C Y
A Y
B X
C Y
C Y
C X
C X
C Z
C Z
C X
A Y
C Z
B Z
C Y
C X
C Y
C Z
C X
A X
A Y
C Z
B Z
C X
C Z
C X
C Z
C X
A X
C Z
A X
B Z
C X";

    let mut win_map: HashMap<&char, i32> = HashMap::new();    
    win_map.insert(&'X', 0);
    win_map.insert(&'Y', 3);
    win_map.insert(&'Z', 6);

    
    let mut rps_map: HashMap<&char, i32> = HashMap::new();    
    rps_map.insert(&'X', 1);
    rps_map.insert(&'Y', 2);
    rps_map.insert(&'Z', 3);
    
    let mut match_map: HashMap<&str, i32> = HashMap::new();    
    match_map.insert("AX", 3);
    match_map.insert("AY", 6);
    match_map.insert("AZ", 0);

    match_map.insert("BX", 0);
    match_map.insert("BY", 3);
    match_map.insert("BZ", 6);

    match_map.insert("CX", 6);
    match_map.insert("CY", 0);
    match_map.insert("CZ", 3);

    let line =s.split('\n');
    // day2part1(rps_map, match_map, line);

    let mut total_score: i32 = 0;
    for i in line{

        let opponent_plays: char =  i.chars().nth(0).unwrap();
        let outcome:char =  i.chars().nth(2).unwrap();

        if (outcome == 'X'){
            if (opponent_plays == 'A') { total_score += 3};
            if (opponent_plays == 'B') { total_score += 1};
            if (opponent_plays == 'C') { total_score += 2};
        }
        else if (outcome == 'Y'){
            if (opponent_plays == 'A') { total_score += 4};
            if (opponent_plays == 'B') { total_score += 5};
            if (opponent_plays == 'C') { total_score += 6};
        }
        else if (outcome == 'Z'){
            if (opponent_plays == 'A') { total_score += 8};
            if (opponent_plays == 'B') { total_score += 9};
            if (opponent_plays == 'C') { total_score += 7};
        }
    }

    println!("{total_score}");

}
fn day2part1(rps_map:HashMap<&char,i32>,match_map:HashMap<&str,i32>,line: std::str::Split<char>){

    let mut total_score: i32 = 0;
    for i in line{
        let opponent_plays: char =  i.chars().nth(0).unwrap();
        let my_play:char =  i.chars().nth(2).unwrap();

        total_score += rps_map.get(&my_play).unwrap();
        println!("rps{total_score}");
        let mut combine_char: String = String::from("");
        combine_char.push(opponent_plays);
        combine_char.push(my_play);

        total_score += match_map.get(&combine_char as &str).unwrap();
        println!("match{total_score}");
    }
    println!("{total_score}");

}