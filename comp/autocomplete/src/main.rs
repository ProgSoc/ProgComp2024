use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::{Hash, Hasher},
    process::exit,
};

use rand::{rngs::StdRng, Rng, SeedableRng};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    assert_eq!(args.len(), 3);

    let mut default_hasher = DefaultHasher::new();
    args[2].hash(&mut default_hasher);
    let seed = default_hasher.finish();

    let article = random_article(seed);

    match args[1].as_str() {
        "generate" => {
            println!("{}", article);
        }
        "validate" => {
            let perms = solve(&article);

            let mut buffer = String::new();
            let _ = std::io::stdin().read_line(&mut buffer);

            let user_perms: usize = buffer
                .trim()
                .parse()
                .graceful_expect("Expected a positive integer.");

            if user_perms == perms {
                exit(0);
            } else {
                exit(1);
            }
        }
        _ => panic!(),
    }
}

trait GracefulExpect<T> {
    fn graceful_expect(self, message: &str) -> T;
}

impl<T, E> GracefulExpect<T> for Result<T, E> {
    fn graceful_expect(self, message: &str) -> T {
        match self {
            Ok(v) => v,
            Err(_) => {
                eprintln!("{}", message);
                exit(1);
            }
        }
    }
}

// fn download_articles() {
//     let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
//
//     const ARTICLES: [&str; 6] = [
//         "Horseshoe crab",
//         "Penitente (snow formation)",
//         "Voynich manuscript",
//         "Glacialisaurus",
//         "Bird intelligence",
//         "Mojibake",
//     ];
//
//     for title in ARTICLES.iter() {
//         let page = wiki.page_from_title(title.to_string());
//
//         let content = page.get_content().unwrap();
//         let content = format_article(&content);
//
//         write_to_file(&content, &format_title(&title));
//     }
// }
//
// fn format_article(text: &str) -> String {
//     text.split_whitespace()
//         .map(|word| word.trim_matches(|c: char| !c.is_alphabetic() || !c.is_ascii()))
//         .map(|word| word.to_lowercase())
//         .filter(|word| !word.is_empty())
//         .collect::<Vec<String>>()
//         .join(" ")
// }
//
// fn format_title(title: &str) -> String {
//     title
//         .split_whitespace()
//         .map(|word| word.trim_matches(|c: char| !c.is_alphabetic()))
//         .map(|word| word.to_lowercase())
//         .filter(|word| !word.is_empty())
//         .collect::<Vec<String>>()
//         .join("-")
// }
//
// fn write_to_file(contents: &str, name: &str) {
//     use std::fs::File;
//     use std::io::Write;
//
//     let mut file = File::create(format!("wiki_articles/{name}.txt")).unwrap();
//     file.write_all(contents.as_bytes()).unwrap();
// }

fn solve(article: &str) -> usize {
    let mut words: HashSet<String> = HashSet::new();
    let mut following_words: HashMap<String, HashSet<String>> = HashMap::new();

    let mut permutations: HashSet<Vec<String>> = HashSet::new();

    let mut previous_word = String::new();

    for word in article.split_whitespace() {
        words.insert(word.to_string());

        if !previous_word.is_empty() {
            if let Some(following) = following_words.get_mut(&previous_word) {
                following.insert(word.to_string());
            } else {
                let mut following = HashSet::new();
                following.insert(word.to_string());
                following_words.insert(previous_word.clone(), following);
            }
        }

        previous_word = word.to_string();
    }

    for word in words {
        let mut sentence = vec![word.clone()];
        build_sentences(&mut sentence, &following_words, &mut permutations);
    }

    permutations.len()
}

fn build_sentences(
    sentence: &mut Vec<String>,
    following_words: &HashMap<String, HashSet<String>>,
    permutations: &mut HashSet<Vec<String>>,
) {
    const SENTENCE_LENGTH: usize = 5;

    if sentence.len() == SENTENCE_LENGTH {
        permutations.insert(sentence.clone());
        return;
    }

    if let Some(following) = following_words.get(sentence.last().unwrap()) {
        for word in following {
            sentence.push(word.clone());
            build_sentences(sentence, following_words, permutations);
            sentence.pop();
        }
    }
}

fn list_directory() -> Vec<String> {
    use std::fs;

    let paths = fs::read_dir("wiki_articles").unwrap();
    let mut files = Vec::new();

    for path in paths {
        let path = path.unwrap().path();
        let path = path.to_str().unwrap().to_string();
        files.push(path);
    }

    files
}

fn random_article(seed: u64) -> String {
    let files = list_directory();
    let file = files[seed as usize % files.len()].clone();
    let contents = std::fs::read_to_string(file).unwrap();

    let tokens: Vec<&str> = contents.split_whitespace().collect();

    const WORDS: usize = 70;

    if tokens.len() < WORDS {
        return tokens.join(" ");
    }

    let mut rng = StdRng::seed_from_u64(seed);
    let start = rng.gen_range(0..tokens.len() - WORDS);

    tokens[start..start + WORDS].join(" ")
}

