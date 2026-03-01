/*
Create a program that:

    Takes a string of text as input
    Splits the text into words (space as separator) // text.split_whitespace().collect();
    Counts the frequency of each word
    Returns the word with the highest frequency and its count

Requirements:

    Use mutable references where appropriate
    Avoid using HashMaps or complex data structures

Solution

fn most_frequent_word(text: &str) -> (String, usize) {
    
    (max_word, max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}

Expected output:

Most frequent word: "the" (3 times)

    Use of mutable references to modify values
    Borrowing of input data
    Working with string slices
    Basic loop structures and indexing
    Ownership rules in function parameters and return values
*/
fn most_frequent_word(text: &str) -> (String, usize){
    let words: Vec<&str> = text.split_whitespace().collect();

    let mut max_word = "";
    let mut max_count = 0;
    //check each word
    for i in 0..words.len(){
        let mut count = 0;

        //count occurrences of words[i]
        for j in 0..words.len(){
            if words[i] == words[j]{
                count += 1;
            }
        }
        // Update max values using mutable references
        if count > max_count{
            max_count = count;
            max_word = words[i];
        }
    }
    // Return owned String (convert from &str)
    (max_word.to_string(), max_count)
}


fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
