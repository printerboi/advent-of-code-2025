
#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    LEFT,
    RIGHT
}

pub struct Dial {
    max_num: i32,
    starting_position: i32,
    pos: i32,
    pub zeros_hit: u32,
}

impl Dial {
    pub fn new(max_num: i32, starting_position: i32) -> Dial {
        Dial {
            max_num,
            starting_position,
            pos: starting_position,
            zeros_hit: 0,
        }
    }

    pub fn turn(&mut self, dir: &Direction, amount: i32) {
        let m: i32 = self.max_num + 1;

        if *dir == Direction::RIGHT {
            self.pos = (self.pos + amount).rem_euclid(m);
        }else{
            self.pos = (self.pos - amount).rem_euclid(m);
        }

        if self.pos == 0 {
            self.zeros_hit += 1;
        }
    }

    pub fn turn_with_overflowcalc(&mut self, dir: &Direction, amount: i32) {
        let mut wraps: u32 = 0;
        let m: i32 = self.max_num + 1;
        let oldpos: i32 = self.pos;

        if *dir == Direction::RIGHT {
            println!("\t + amount: {:?}", amount);
            self.pos = (self.pos + amount).rem_euclid(m);

            let mut passes = (oldpos + amount) / m;
            if oldpos == 0 && passes > 0 {
                passes -= 1;
            }
            wraps = passes as u32;
        } else {
            println!("\t - amount: {:?}", amount);
            self.pos = (self.pos - amount).rem_euclid(m);

            let mut passes = if amount > oldpos {
                ((amount - oldpos - 1) / m) + 1
            } else {
                0
            };
            if oldpos == 0 && passes > 0 {
                passes -= 1;
            }
            wraps = passes as u32;
        }

        if self.pos == 0 && wraps > 0 {
            wraps -= 1;
        }

        if self.pos == 0 {
            self.zeros_hit += 1;
        }

        self.zeros_hit += wraps;
        println!("\t wraps: {:?}", wraps);
    }

    pub fn print(&self) {
        println!("\t  pos: {:?}", self.pos);
    }

    pub fn reset(&mut self) {
        self.pos = self.starting_position;
        self.zeros_hit = 0;
    }
}