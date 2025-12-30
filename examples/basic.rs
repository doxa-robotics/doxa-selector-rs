use autons::prelude::*;
use doxa_selector::{route, DoxaSelect};
use vexide::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Category {
    RouteA,
    RouteB,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::RouteA => write!(f, "Route A"),
            Category::RouteB => write!(f, "Route B"),
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
                route!(Category::RouteA, Robot::route_1),
                route!(Category::RouteB, Robot::route_2),
            ],
        ))
        .await;
}
