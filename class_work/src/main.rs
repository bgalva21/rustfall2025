// fn append_region(word: &mut String){
//     word.push_str("RGV");
// }


// fn main(){
//     let mut x =  "UT".to_string();
//     append_region(&mut x);
//     println!("{}",x);
// }

fn concat_strings(s1: &String, s2: &String) -> String {
    // Your code here
    let mut new_word = String::new();
    new_word.push_str(s1);
    new_word.push_str(s2);
    new_word
}

fn clone_and_modify(s: &String) -> String {
    // Your code here
    let mut clone = s.clone();
    clone.push_str("World!");
    clone
}

fn sum(total: &mut i32, low: i32, high: i32) {
    // Write your code here!
    *total = 0;
    for i in low..=high{
        *total += i;
    }
}

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"


    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"

    let mut total = 0;
    sum(&mut total,0,100);
    println!("Result = {}", total);
}