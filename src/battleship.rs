extern crate rand;

pub mod battleship {

    pub const MAP_SIZE: usize = 5;
    pub const SHIP_SIZES: [u8; 4] = [1, 2, 3, 4];
    pub const NUM_SHIPS: usize = 4;

    pub const H_SHIPS: [(&str,&str);4] = [("Submarine", "O"), ("Destroyer", "<>"), ("Cruiser", "<=>"), ("Battleship", "<==>")] ;
    pub const V_SHIPS: [(&str,&str);4] = [("Submarine", "O"), ("Destroyer", "˄˅"), ("Cruiser", "˄ǁ˅"), ("Battleship", "˄ǁǁ˅")] ;

    pub struct Game {
        back_map: Vec<Vec<char>>,
        play_map: Vec<Vec<char>>,
        points: u8,
        misses: u8,
        max_misses: u8,
        max_points: u8,
    }

    impl Game {
        pub fn new() -> Self {
            let mut back_map = vec![vec!['#'; MAP_SIZE]; MAP_SIZE];
            let play_map = vec![vec!['#'; MAP_SIZE]; MAP_SIZE];
            place_random_ships(&mut back_map);
            let _max_misses = MAP_SIZE as u8;
            let _max_points = back_map.iter().flatten().filter(|&c| *c != '#').count() as u8;
            Game { back_map, play_map, points: 0, misses: 0, max_misses: _max_misses, max_points: _max_points }
        }
        pub fn print_map(&self) {
            print!(" \t");
            for i in 0..MAP_SIZE {
                print!("{}  ", ('a' as u8 + i as u8) as char);
            }
            println!("");
            for i in 0..MAP_SIZE {
                for j in 0..MAP_SIZE {
                    if j == 0 {
                        print!("{}\t", i + 1);
                    }
                    print!("{}  ", self.play_map[i][j]);
                }
                println!("");
            }
            println!("Points: {}/{}", self.points, self.max_points);
            println!("Misses: {}/{}", self.misses, self.max_misses);
        }
        pub fn process_shot(&mut self, y: usize, x: usize) -> bool {
            if self.back_map[x][y] == '#' {
                self.play_map[x][y] = '_';
                self.misses += 1;
                return false;
            } else {
                self.play_map[x][y] = self.back_map[x][y];
                self.points += 1;
                return true;
            }
        }
        pub fn is_game_over(&self) -> bool {
            self.points == self.max_points || self.misses == self.max_misses
        }
        pub fn reveal_map(&mut self) {
            for i in 0..MAP_SIZE {
                for j in 0..MAP_SIZE {
                    if self.back_map[i][j] != '#' {
                        self.play_map[i][j] = self.back_map[i][j];
                    }
                }
            }
            self.print_map();
        }
    }

    fn place_random_ships(map: &mut Vec<Vec<char>>) {
        for _i in 0..NUM_SHIPS {
            let mut x: usize;
            let mut y: usize;
            let mut vertical: bool;
            let mut size: usize;
            loop {
                x = rand::random::<usize>() % MAP_SIZE;
                y = rand::random::<usize>() % MAP_SIZE;
                size = rand::random::<usize>() % SHIP_SIZES.len();
                vertical = rand::random::<bool>();

                if check_ship(map, x, y, SHIP_SIZES[size], vertical) {
                    break;
                }
            }
            add_ship(map, x, y, SHIP_SIZES[size], vertical);
        }
    }
    fn check_ship(map: &Vec<Vec<char>>, x: usize, y: usize, size: u8, vertical: bool) -> bool {
        if vertical {
            if x + size as usize > MAP_SIZE {
                return false;
            }
            for i in x..x + size as usize {
                if map[i][y] != '#' {
                    return false;
                }
            }
        } else {
            if y + size as usize > MAP_SIZE {
                return false;
            }
            for i in y..y + size as usize {
                if map[x][i] != '#' {
                    return false;
                }
            }
        }
        return true;
    }
    fn add_ship(map: &mut Vec<Vec<char>>, x: usize, y: usize, size: u8, vertical: bool) {
        let mut count = 0;
        if vertical {
            for i in x..x + size as usize {
                map[i][y] = V_SHIPS[size as usize -1].1.chars().nth(count).unwrap();
                count += 1;
            }
        } else {
            for i in y..y + size as usize {
                map[x][i] = H_SHIPS[size as usize -1].1.chars().nth(count).unwrap();
                count += 1;
            }
        }
    }
}
