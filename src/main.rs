use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Replace "your_file.txt" with the path to your text file
    let file = File::open("file.txt")?;
    let reader = io::BufReader::new(file);

    // Vector to store the results for each line
    let mut possible_games = 0;
    let mut game_possible:bool ; 
    let mut id =0 ; 
    // Iterate over lines in the file
    for line in reader.lines() {
        // Handle each line as needed
     id  += 1 ;  
        game_possible = true  ; 
        match line {
            Ok(text) => {
                // Find the index of ":" and take the substring after it
                let after_colon: String = text.split(':').skip(1).collect();
                // Split the remaining string by ","
                let values: Vec<&str> = after_colon.split(';').flat_map(|s| s.split(',')).collect();
                // Process each value in the values array
                for part in &values {
                    // Split each value by space
                    let value_parts: Vec<&str> = part.split_whitespace().collect();

                    // Parse the first substring as an integer
                    if let Some(first_substring) = value_parts.get(0) {
                        if let Ok(parsed_int) = first_substring.parse::<i32>() {
                            // Keep the second substring
                            if let Some(second_substring) = value_parts.get(1) {
                                // Print or process the parsed integer and second substring
                                match *second_substring{
                                    "red" if parsed_int > 12 => game_possible =false ,
                                    "blue" if parsed_int > 14 => game_possible =false,
                                    "green" if parsed_int > 13 => game_possible =false,
                                    _ => {} // Handle other cases or do nothing
                                }
                                println!("Parsed Integer: {}, Second Substring: {} ", parsed_int, second_substring);
                            }
       
                        }
                    }
                }
            }
            Err(err) => eprintln!("Error reading line: {}", err),
        }
 
if game_possible == true{
possible_games+=id ; 
}


}
println!("possible_games{} ",possible_games) ;
                
    Ok(())
}