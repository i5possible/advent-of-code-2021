use std::fs;

fn main() {
    let filename = "day2.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let moves: Vec<&str> = contents
        .split('\n')
        .collect::<Vec<&str>>();

    let product1: i32 = moves.iter().map(|move_str| {
        let move_vec: Vec<&str> = move_str.split(' ').collect();

        let movement = match move_vec[0] {
            "forward" => (0, move_vec[1].parse::<i32>().unwrap()),
            "down" => (move_vec[1].parse::<i32>().unwrap(), 0),
            "up" => (move_vec[1].parse::<i32>().unwrap() * -1, 0),
            _other => (0, 0),
        };
        movement
    }).reduce(|acc, cur| {
        (acc.0 + cur.0, acc.1 + cur.1)
    }).map(|movement| {
        movement.0 * movement.1
    }).unwrap();

    let mut position = (0, 0, 0);
    for move_str in moves {
        let move_vec: Vec<&str> = move_str.split(' ').collect();
        position = match move_vec[0] {
            "forward" => (position.0, position.1 + move_vec[1].parse::<i32>().unwrap(), position.2 + position.0 * move_vec[1].parse::<i32>().unwrap()),
            "down" => (position.0 + move_vec[1].parse::<i32>().unwrap(), position.1, position.2),
            "up" => (position.0 - move_vec[1].parse::<i32>().unwrap(), position.1, position.2),
            _other => position,
        };
        println!("{:?}", position);
    }
    let product2 = position.1 * position.2;
   
   println!("Product1 is {}", product1);
   println!("Product2 is {}", product2);
}
