#![no_std]
#![no_main]

extern crate alloc;

use alloc::{
    boxed::Box,
    collections::btree_map::BTreeMap,
    vec::{self, Vec},
};
use core::{
    any::Any,
    error::Error,
    fmt::{Debug, Display},
    future::Future,
    time::Duration,
};

use async_trait::async_trait;
use doxa_selector_rs::{AutonRoutine, CompeteWithSelector, CompeteWithSelectorExt as _};
use vexide::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Category {
    RouteA,
    RouteB,
}

impl Display for Category {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Category::RouteA => write!(f, "RouteA"),
            Category::RouteB => write!(f, "RouteB"),
        }
    }
}

struct RouteA;

#[async_trait]
impl AutonRoutine<TestRobot> for RouteA {
    fn name(&self) -> &'static str {
        "RouteA"
    }

    fn description(&self) -> &'static str {
        "This is Route A"
    }

    async fn run(&self, context: TestRobot) -> Result<(), Box<dyn Error>> {
        println!("RouteA run");
        Ok(())
    }
}

struct RouteB;

#[async_trait]
impl AutonRoutine<TestRobot> for RouteB {
    fn name(&self) -> &'static str {
        "RouteB"
    }

    fn description(&self) -> &'static str {
        "This is Route B"
    }

    async fn run(&self, context: TestRobot) -> Result<(), Box<dyn Error>> {
        println!("RouteB run");
        Ok(())
    }
}

struct TestRobot {}

impl CompeteWithSelector<Category> for TestRobot {
    async fn driver(&mut self) {
        println!("Driver control");
    }

    fn autonomous_routes(&self) -> BTreeMap<Category, &[&dyn AutonRoutine<TestRobot>]> {
        let mut map: BTreeMap<Category, &[&dyn AutonRoutine<TestRobot>]> = BTreeMap::new();
        map.insert(Category::RouteA, &[&RouteA]);
        map.insert(Category::RouteB, &[&RouteB]);
        map
    }
}

#[vexide::main]
async fn main(peripherals: Peripherals) {
    let robot = TestRobot {};
    robot.compete_with_selector(peripherals.display).await;
}
