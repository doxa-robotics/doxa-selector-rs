use autons::prelude::*;
use doxa_selector::{route, DoxaSelect};
use vexide::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Category {
    Category1,
    Category2,
    Category3,
    Category4,
    Category5,
    Category6,
    Category7,
    Category8,
    Category9,
    Category10,
    Category11,
    Category12,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::Category1 => write!(f, "Category 1"),
            Category::Category2 => write!(f, "Category 2"),
            Category::Category3 => write!(f, "Category 3"),
            Category::Category4 => write!(f, "Category 4"),
            Category::Category5 => write!(f, "Category 5"),
            Category::Category6 => write!(f, "Category 6"),
            Category::Category7 => write!(f, "Category 7"),
            Category::Category8 => write!(f, "Category 8"),
            Category::Category9 => write!(f, "Category 9"),
            Category::Category10 => write!(f, "Category 10"),
            Category::Category11 => write!(f, "Category 11"),
            Category::Category12 => write!(f, "Category 12"),
        }
    }
}

struct Robot {}

impl Robot {
    async fn route_1(&mut self) {
        println!("Route 1");
    }
    async fn route_2(&mut self) {
        println!("Route 2");
    }
}

impl SelectCompete for Robot {}

#[vexide::main]
async fn main(peripherals: Peripherals) {
    let robot = Robot {};

    robot
        .compete(DoxaSelect::new(
            peripherals.display,
            [
                route!(Category::Category1, Robot::route_1),
                route!(Category::Category2, Robot::route_2),
                route!(Category::Category3, Robot::route_1),
                route!(Category::Category4, Robot::route_2),
                route!(Category::Category5, Robot::route_1),
                route!(Category::Category6, Robot::route_2),
                route!(Category::Category7, Robot::route_1),
                route!(Category::Category8, Robot::route_2),
                route!(Category::Category9, Robot::route_1),
                route!(Category::Category10, Robot::route_2),
                route!(Category::Category11, Robot::route_1),
                route!(Category::Category12, Robot::route_2),
            ],
        ))
        .await;
}
