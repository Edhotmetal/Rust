use std::io;
fn main()
{
    println!("Please enter some text");

    let mut input = String::new();

    // get input from user
    loop
    {
        match io::stdin().read_line(&mut input)
        {
            Ok(text) => text,
            Err(_) =>
            {
                println!("Error reading input!");
                continue;
            }
        };
       break;
    }

    println!("Your input: {}", input);

    println!("The first word in the string is {}", first_word(&input));

    println!("All of the words in the string:");

    for(i, &word) in slice(&input).iter().enumerate()
    {
        println!("{}", word);
    }
}

fn first_word(s: &String) -> &str
{
    let bytes = s.as_bytes(); // convert string into an array of bytes
    let mut index: usize = s.len()-2;

    for (i, &item) in bytes.iter().enumerate() // iterate over the array
    {
        if item == b' ' // byte literal
        {
            index = i;
            break;
        }
    }

    return &s[0..index];
}

// Accepts a string and splits it by the provided delimiter
// The default delimiter is a space
fn slice(s: &String)-> Vec<&str>
{
    let bytes = s.as_bytes(); // convert the string into an array of bytes
    let mut words = Vec::new(); // The list of delimited words
    let mut index: usize = 0; // Used to store the beginning of each word when adding to the vector

    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' ' // If a delimiter is reached
        {
            words.push(&s[index..i]); // Add the word to the vector
            index = i+1; // Record the beginning of the next word
        }
    }
    words.push(&s[index..s.len()-2]); // Add the final word
    
    return words;
}
