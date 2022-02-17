use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter() // <_,_> => type of data
                                        .zip(initial_scores.into_iter()) // zip - create a vector of tuples (Blue, 10) and (Yellow, 50) 
                                        .collect(); // turn that vector of tuples into a hash map

    // accessing values in Hash Map
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // return Option<&T> for matching key in the hash map
    println!("score: {:#?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // replace (overwrite)
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // discard new value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow"))
            .or_insert(50); // return a mutable reference to the value for the corresponding Entry key if exists else, inserts the parameter and returns a mutable reference of the new value
    scores.entry(String::from("Blue")).or_insert(25);

    println!("{:?}", scores);

    // Update value based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // mutable reference to the value
        *count += 1;
    }

    println!("{:?}", map);

}
