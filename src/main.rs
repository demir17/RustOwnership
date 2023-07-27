fn main() {
    let mut first_word = String::from("Hello");
    let second_word = " World";
    let result = concatenate_strings(&first_word, second_word);


    println!("{}",result );

}

fn concatenate_strings(s: &str, d : &str) -> String{
    let mut result = String::from(s);

    result.push_str(d);
    result
}

