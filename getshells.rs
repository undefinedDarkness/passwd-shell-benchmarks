use std::fs;
use std::collections::HashMap;

fn print_map(map: &mut HashMap<&str, i32>) {
    for (key, value) in map {
        println!("{} / {}", key, value);
    }
}

fn main() {

    let mut shellmap = HashMap::new();
    let file_path = "passwd";
    let contents: String = fs::read_to_string(file_path).expect("unable to open file");

    for line in contents.lines() {
        let shell = line.rsplit_once(':').unwrap().1;
        shellmap.entry(shell)
            .and_modify(|count| *count+=1)
            .or_insert(1);
    }
    print_map(&mut shellmap);
}
