use std::fs::read_to_string;

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

fn setup(file_name: &str) -> Vec<Race> {
    let contents = read_to_string(file_name).expect("Something went wrong reading the file");
    let mut contents = contents.lines();
    let times = contents.next().unwrap().split(":").nth(1).unwrap().trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let distances = contents.next().unwrap().split(":").nth(1).unwrap().trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let mut races: Vec<Race> = Vec::new();
    for i in 0..times.len() {
        races.push(Race {
            time: times[i],
            distance: distances[i],
        });
    }
    races
}

fn get_winning_strategies(race: Race) -> usize {
    let mut total: usize = 0;
    for speed in 1..race.time {
        let time_remaining = race.time - speed;
        if time_remaining * speed > race.distance {
            total += 1;
        }
    }
    total
}


fn solution(races: Vec<Race>) -> usize {
    let mut winning_race_strategies = Vec::<usize>::new();
    for race in races {
        winning_race_strategies.push(get_winning_strategies(race))
    }
    winning_race_strategies.iter().fold(1, |acc, x| acc * x)
}

fn main() {
    let example_races = setup("example.txt");
    let input_races = setup("input.txt");
    println!("{}", solution(example_races));
    println!("{}", solution(input_races));

    let example_2_races = setup("example_2.txt");
    let input_2_races = setup("input_2.txt");
    println!("{}", solution(example_2_races));
    println!("{}", solution(input_2_races)); 
}
