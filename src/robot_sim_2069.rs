
#[allow(dead_code)]
#[derive(Debug)]
struct Robot {
    position: i32,
    width: i32,
    height: i32,
}

enum Direction {
    East,
    North,
    West,
    South,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl Robot {

    fn new(width: i32, height: i32) -> Self {
        Robot {position: 0, width: width, height: height}
    }
    
    fn step(&mut self, num: i32) {
        let perimeter = 2 * (self.width + self.height - 2);
        self.position = (self.position + num) % perimeter;
        debug_assert!((0..self.width).contains(&self.get_pos()[0]), "position: {}", self.position);
        debug_assert!((0..self.height).contains(&self.get_pos()[1]), "position: {}", self.position);
    }
    
    fn get_pos(&self) -> Vec<i32> {
        let half_perimeter = self.width + self.height - 2;
        assert!((0..half_perimeter * 2).contains(&self.position));
        let (x, y) = match self.get_direction() {
            Direction::East => {
                (self.position, 0)
            }
            Direction::North => {
                (self.width - 1, self.position % self.bottom_right_index())
            }
            Direction::West => {
                (self.width - 1 - (self.position % self.top_right_index()), self.height - 1)
            }
            Direction::South => {
                (0, self.height - 1 - (self.position % self.top_left_index()))
            }
        };
        vec!(x, y)
    }

    fn get_direction(&self) -> Direction {
        let half_perimeter = self.width + self.height - 2;
        assert!(self.position < 2 * half_perimeter);
        match self.position {
            p if p <= self.bottom_right_index() => {
                Direction::East
            }
            p if p <= self.top_right_index() => {
                Direction::North
            }
            p if p <= self.top_left_index() => {
                Direction::West
            }
            _ => {
                Direction::South
            }
        }
    }

    fn bottom_right_index(&self) -> i32 {
        self.width - 1
    }

    fn top_right_index(&self) -> i32 {
        self.width + self.height - 2
    }

    fn top_left_index(&self) -> i32 {
        2 * self.width + self.height - 3
    }
    
    fn get_dir(&self) -> String {
        String::from(match self.get_direction() {
            Direction::East => "East",
            Direction::North => "North",
            Direction::West => "West",
            Direction::South => "South",
        })
    
    }
}

#[test]
fn official_test() {
    let mut robot = Robot::new(6, 3); 
    robot.step(2);  // It moves two steps East to (2, 0), and faces East.
    robot.step(2);  // It moves two steps East to (4, 0), and faces East.
    assert_eq!(vec!(4, 0), robot.get_pos()); // return [4, 0]
    assert_eq!("East", robot.get_dir().as_str()); // return "East"
    robot.step(2);  // It moves one step East to (5, 0), and faces East.
                    // Moving the next step East would be out of bounds, so it turns and faces North.
                    // Then, it moves one step North to (5, 1), and faces North.
    assert_eq!(vec!(5, 1), robot.get_pos());
    assert_eq!("North", robot.get_dir().as_str()); 
    robot.step(1);  // It moves one step North to (5, 2), and faces North (not West).
    assert_eq!(vec!(5, 2), robot.get_pos());
    assert_eq!("North", robot.get_dir().as_str()); 
    robot.step(4);  // Moving the next step North would be out of bounds, so it turns and faces West.
                    // Then, it moves four steps West to (1, 2), and faces West.
    assert_eq!(vec!(1, 2), robot.get_pos()); // return [1, 2]
    assert_eq!("West", robot.get_dir().as_str()); // return "West"
}

#[test]
fn error1() {
    let mut robot = Robot::new(8, 11); 
    robot.position = 8;
    assert_eq!(vec!(7, 1), robot.get_pos());
    robot.step(44);
    robot.step(36);
    assert_eq!(vec!(4, 10), robot.get_pos());
    assert_eq!("West", robot.get_dir().as_str());
}

#[test]
fn error2() {
    let mut robot = Robot::new(20, 14); 
    robot.step(207);
    assert_eq!(vec!(15, 0), robot.get_pos());
    robot.step(8);
    assert_eq!(vec!(19, 4), robot.get_pos());
    assert_eq!("North", robot.get_dir().as_str());
    robot.step(11);
    assert_eq!(vec!(17, 13), robot.get_pos());
    assert_eq!("West", robot.get_dir().as_str());
    robot.step(18);
    assert_eq!(vec!(0, 12), robot.get_pos());
    assert_eq!("South", robot.get_dir().as_str());
}

#[test]
fn error3() {
    let mut robot = Robot::new(20, 14); 
    robot.step(52);
    assert_eq!(vec!(0, 12), robot.get_pos());
}

#[test]
fn error4() {
    let mut robot = Robot::new(6, 3); 
    robot.step(7);
    assert_eq!(vec!(5, 2), robot.get_pos());
    assert_eq!("North", robot.get_dir().as_str());
}

#[test]
fn error5() {
    let mut robot = Robot::new(20, 13); 
    robot.step(12+21+17);
    assert_eq!("West", robot.get_dir().as_str());
    assert_eq!(vec!(0, 12), robot.get_pos());
}

#[test]
fn error6() {
    let mut robot = Robot::new(8, 11); 
    // Perimeter: 34
    robot.step(32);
    assert_eq!(vec!(0, 2), robot.get_pos());
    assert_eq!("South", robot.get_dir().as_str());
}