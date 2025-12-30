use autons::prelude::*;
use doxa_selector::{route, DoxaSelect};
use vexide::prelude::*;

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
            [route!(Robot::route_1), route!(Robot::route_2)],
        ))
        .await;
}
