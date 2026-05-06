#[derive(Debug)]
struct PlayerPoint {
    
    x: f32,
    y: f32
}

fn print_player_point(pp: &PlayerPoint) {

    println!("Player x point: {:}", pp.x);
    println!("Player y point: {:}", pp.y);
}

fn main() {
     
     let mut point1 = PlayerPoint { x: 2.12, y: 3.24};

     print_player_point(&point1);

     point1.x = 4.35;

     point1.y = -3.46;

     print_player_point(&point1);
}

// Player x point: 2.12
// Player y point: 3.24
// Player x point: 4.35
// Player y point: -3.46
