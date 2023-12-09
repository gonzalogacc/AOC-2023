
#[derive(Default)]
struct Batch {
    red: i32,
    green: i32,
    blue: i32,
}

impl Batch {
    pub fn new(batch_description: String) -> Self {
        for color in batch_description.split(",") {
                println!("{}", color);
        }
        Self {red: 0, green: 0, blue: 0}
    }
}

#[test]
fn test_parse_batch(){
    let batch_string: String = String::from("3 blue, 4 red");
    let batch: Batch = Batch::new(batch_string);
    println!("{}", batch);

    assert_eq!(batch.red, 3);
    assert_eq!(batch.blue, 4);
    assert_eq!(batch.green, 0);
}

struct Game {
    id: i32,
    batches: Vec<Batch>
}

// impl Game {
//     fn new(game_string: i32) -> Self {
//         Self.red
//     }
// }
fn main() {
    println!("Hello, world!");
}



// #[test]
// fn test() {
//     Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
//     Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
//     Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
//     Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
//     Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
// }