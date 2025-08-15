use std::collections::HashMap;
use std::env::args;
use std::io::stdin;
use std::process::exit;
use colored::Colorize;


fn main() {
    let mut alphabet = HashMap::new();
    alphabet.insert(0, 'a');
    alphabet.insert(1, 'b');
    alphabet.insert(2, 'c');
    alphabet.insert(3, 'd');
    alphabet.insert(4, 'e');
    alphabet.insert(5, 'f');
    alphabet.insert(6, 'g');
    alphabet.insert(7, 'h');
    alphabet.insert(8, 'i');
    alphabet.insert(9, 'j');
    alphabet.insert(10, 'k');
    alphabet.insert(11, 'l');
    alphabet.insert(12, 'm');
    alphabet.insert(13, 'n');
    alphabet.insert(14, 'o');
    alphabet.insert(15, 'p');
    alphabet.insert(16, 'q');
    alphabet.insert(17, 'r');
    alphabet.insert(18, 's');
    alphabet.insert(19, 't');
    alphabet.insert(20, 'u');
    alphabet.insert(21, 'v');
    alphabet.insert(22, 'w');
    alphabet.insert(23, 'x');
    alphabet.insert(24, 'y');
    alphabet.insert(25, 'z');

    let mut cracking_key = 0;

    let common_short_words = vec![
        "i",
        "the",
        "an",
        "full",
        "bug",
        "of",
        "for",
        "you",
        "not",
        "are",
        "new",
        "was",
        "can",
        "has",
        "out",
        "use",
        "may",
        "our",
        "see",
        "his",
        "him",
        "her",
        "any",
        "but",
        "and",
        "all",
        "one",
        "two",
        "we",
    ];


    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("
USAGE:
CrackaCesar cipher text
        ");
        exit(1);
    }
    let mut cipher_vec = args.clone();
    cipher_vec.remove(0);
    let cipher_text = args[1..].join(" ");
    let mut short_words = Vec::new();
    for word in cipher_vec{
        if word.len() < 4{
            short_words.push(word.clone());
        }
    }
    let mut cracked = false;
    let mut failed_decrypt_words = Vec::new();
    for key in 0..26{
        let mut decrypted_words = Vec::new();
        for oword in short_words.clone(){
            let word = oword.to_lowercase();
            let mut addition_word  = String::new();
            let chars = word.chars();
            for letter in chars{
                if letter.is_ascii_alphabetic(){
                    let mut letter_orig_index = 0;
                    for (key, value) in alphabet.iter(){
                        if value == &letter{
                            letter_orig_index = *key;
                        }
                    }
                    let mut add_letter_index:i32 = letter_orig_index + key;
                    if add_letter_index > 25{
                        add_letter_index = add_letter_index - 26;
                    }
                    addition_word.push(alphabet[&add_letter_index]);
                }
            }
            decrypted_words.push(addition_word.clone());
            failed_decrypt_words.push(format!("{} => {}", key, &addition_word));
        }
        let success_num = decrypted_words.len();
        let mut valid_num: usize = 1;
        for word in &decrypted_words{
            if common_short_words.contains(&word.as_str()){
                valid_num += 1;
            }
        }
        if valid_num >= success_num{
            println!("{}\n", "key found!".green());
            cracked = true;
            cracking_key = key;
        }
        else if valid_num >= success_num / 2{
            println!("{}", "possible key found!".green());
            println!("{} => {}", key, decrypted_words.join("\n"));
            let mut crack_res = String::new();
            println!("try to decypt the whole string with this key?");
            stdin().read_line(&mut crack_res).unwrap();
            if crack_res.to_lowercase().contains("y"){
                cracked = true;
                cracking_key = key;
            }
        }
    }
    if !cracked{
        println!("{}", "couldn't automatically determine a cracking key...".red());
        println!("{}", failed_decrypt_words.join("\n\n"));
        let mut key_response = String::new();
        println!("key that resulted in real words? (enter no if none are)");
        stdin().read_line(&mut key_response).unwrap();
        if key_response.to_lowercase().contains("n"){
            println!("{}", "sorry about that, we can't seem to find the right key...".red());
            exit(1);
        }
        cracking_key = key_response.trim().parse().unwrap();
    }

    let mut cracked_string = String::new();
    for oletter in cipher_text.chars(){
        if oletter.is_alphabetic(){
            let letter = oletter.to_ascii_lowercase();
            let mut orig_index = 0;
            for (key, value) in alphabet.iter(){
                if value == &letter{
                    orig_index = *key;
                }
            }
            let mut new_index: i32 = orig_index + cracking_key;
            if new_index > 25{
                new_index -= 26;
            }
            let new_letteer = alphabet[&new_index];
            cracked_string.push(new_letteer);
        }
        else{
            cracked_string.push(oletter);
        }
    }
    println!("{}\n{}", "Decdrypted cipher:".green() ,cracked_string);
}
