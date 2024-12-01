fn main() {
    let test = vec!["one", "two", "three"];
    let index = test.iter().position(|&r| r == "two").unwrap();
    println!("{}", index);
}