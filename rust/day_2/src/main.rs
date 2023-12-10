use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
#[derive(Default)]
struct Batch {
    red: i32,
    green: i32,
    blue: i32,
}

impl Batch {
    pub fn new(batch_description: &str) -> Self {
        let mut red: i32 = 0;
        let mut green: i32 = 0;
        let mut blue: i32 = 0;
        for color in batch_description.split(",") {
            let mut iter = color.trim().splitn(2, ' ');
            let quant: i32 = iter.next().unwrap().parse::<i32>().unwrap();
            let color: &str = iter.next().unwrap();
            match color {
                "red" => red = quant,
                "green" => green = quant,
                "blue" => blue = quant,
                _ => {}
            }
        }
        Self { red, green, blue }
    }

    fn is_valid_batch(&self, other: & Batch) -> bool {
        if self.red > other.red || self.green > other.green || self.blue > other.blue {
            return false;
        }
        return true
    }
}


#[derive(Debug)]
struct Game {
    id: i32,
    batches: Vec<Batch>
}

impl Game {
    fn new(game_string: &str) -> Self {
        let mut iter = game_string.trim().splitn(2, ":");
        let mut header_it = iter.next().unwrap().splitn(2, ' ');
        header_it.next();
        let id: i32 = header_it.next().unwrap().parse::<i32>().unwrap();
        let batch_string = iter.next().unwrap();

        let mut batches: Vec<Batch> = Vec::new();
        for batch in batch_string.split(";") {
            batches.push(Batch::new(batch));
        }
        Self {id, batches}
    }

    fn is_possible(&self, bag: & Batch) -> bool {
        for batch in &self.batches {
           if !batch.is_valid_batch(bag) {
               return false;
           }
        }
        true
    }

    fn min_dice_set(&self) -> Batch {
        let red = self.batches.iter().map(|b| b.red).max().unwrap();
        let green = self.batches.iter().map(|b| b.green).max().unwrap();
        let blue = self.batches.iter().map(|b| b.blue).max().unwrap();
        Batch {red, green, blue}
    }

    fn compute_dice_power(&self) -> i32 {
        let min_dice_set = self.min_dice_set();
        min_dice_set.red * min_dice_set.green * min_dice_set.blue
    }

}


fn load_games(filename: String) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines(){
        match line {
            Ok(line) => games.push(Game::new(&line)),
            Err(e) => println!("Error reading file: {}", e),
        }
    }
    games
}


fn games_filter(games: & Vec<Game>, bag: & Batch) -> i32 {
    let mut id_sum: i32 = 0;
    for game in games {
        if game.is_possible(bag) {
            id_sum += game.id;
        }
    }
    id_sum
}

fn games_total_power(games: & Vec<Game>) -> i32 {
    let mut power_sum: i32 = 0;
    for game in games {
        power_sum += game.compute_dice_power();
    }
    power_sum
}

fn main() {
    let games = load_games(String::from("./input.txt"));
    let bag: Batch = Batch::new("12 red, 13 green, 14 blue");
    let total = games_filter(& games, &bag);
    let power_sum = games_total_power(&games);

    println!("Games filer: {}", total);
    println!("Power sum: {}", power_sum);
}


#[test]
fn test_valid_game_sum(){
    let games = load_games(String::from("./test_input.txt"));
    let bag: Batch = Batch::new("12 red, 13 green, 14 blue");
    let total = games_filter(& games, &bag);
    assert_eq!(total, 8);
}

#[test]
fn test_parse_batch() {
    let batch_string = "3 blue, 4 red";
    let batch: Batch = Batch::new(batch_string);
    assert_eq!(batch.red, 4);
    assert_eq!(batch.green, 0);
    assert_eq!(batch.blue, 3);

    let batch_string2= "1 red, 2 green, 6 blue";
    let batch: Batch = Batch::new(batch_string2);
    assert_eq!(batch.red, 1);
    assert_eq!(batch.green, 2);
    assert_eq!(batch.blue, 6);

    let batch_string2  = "2 green";
    let batch: Batch = Batch::new(batch_string2);
    assert_eq!(batch.red, 0);
    assert_eq!(batch.green, 2);
    assert_eq!(batch.blue, 0);
}

#[test]
fn test() {
    let game1 = Game::new("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
    assert_eq!(game1.id, 1);
    assert_eq!(game1.batches.len(), 3);
    assert_eq!(game1.batches[0].red, 4);
    assert_eq!(game1.batches[0].green, 0);
    assert_eq!(game1.batches[0].blue, 3);

    assert_eq!(game1.batches[1].red, 1);
    assert_eq!(game1.batches[1].green, 2);
    assert_eq!(game1.batches[1].blue, 6);

    let game2 = Game::new("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
    assert_eq!(game2.id, 2);
    assert_eq!(game2.batches.len(), 3);
    assert_eq!(game2.batches[0].red, 0);
    assert_eq!(game2.batches[0].green, 2);
    assert_eq!(game2.batches[0].blue, 1);

    assert_eq!(game2.batches[1].red, 1);
    assert_eq!(game2.batches[1].green, 3);
    assert_eq!(game2.batches[1].blue, 4);

    let game3 = Game::new("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");
    assert_eq!(game3.id, 3);
    assert_eq!(game3.batches.len(), 3);

    let game4 = Game::new("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red");
    assert_eq!(game4.id, 4);
    assert_eq!(game4.batches.len(), 3);

    let game5 = Game::new("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
    assert_eq!(game5.id, 5);
    assert_eq!(game5.batches.len(), 2);
}


#[test]
fn test_read_file() {
    let games: Vec<Game> = load_games(String::from("./test_input.txt"));
    assert_eq!(games[1].id, 2);
    assert_eq!(games[1].batches[0].green, 2);
    assert_eq!(games[1].batches[0].blue, 1);
    assert_eq!(games[1].batches[0].red, 0);

    assert_eq!(games[2].id, 3);
    assert_eq!(games[2].batches[2].red, 1);
    assert_eq!(games[2].batches[2].green, 5);
    assert_eq!(games[2].batches[2].blue, 0);
}


#[test]
fn test_dice_set_power(){
    let game1 = Game::new("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
    assert_eq!(game1.compute_dice_power(), 48);

    let game2 = Game::new("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
    assert_eq!(game2.compute_dice_power(), 12);

    let game3 = Game::new("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");
    assert_eq!(game3.compute_dice_power(), 1560);

    let game4 = Game::new("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red");
    assert_eq!(game4.compute_dice_power(), 630);

    let game5 = Game::new("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
    assert_eq!(game5.compute_dice_power(), 36);
}

#[test]
fn test_min_dice_set() {
    let game1 = Game::new("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
    assert_eq!(game1.min_dice_set().red, 4);
    assert_eq!(game1.min_dice_set().green, 2);
    assert_eq!(game1.min_dice_set().blue, 6);
}