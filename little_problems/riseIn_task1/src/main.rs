fn concate_strings(str1: &str, str2: &str) -> String
{
    let mut result = String::from(str1);
    result.push_str(str2);
    result
}

fn main(){
    let string1 = "Rise";
    let string2 = "In!";
    let concatenated_string = concate_strings(string1, string2);
    println!("{}", concatenated_string);
}