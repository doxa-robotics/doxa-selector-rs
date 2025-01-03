#![no_std]
#![no_main]

extern crate alloc;

use alloc::{boxed::Box, collections::btree_map::BTreeMap, rc::Rc, string::ToString, vec::Vec};
use core::{
    error::Error,
    fmt::{Debug, Display},
};

use async_trait::async_trait;
use doxa_selector_rs::{AutonRoutine, CompeteWithSelector, CompeteWithSelectorExt as _};
use vexide::{core::sync::Mutex, prelude::*};

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
    type Return = Result<(), Box<dyn Error>>;

    fn name(&self) -> &'static str {
        "RouteA"
    }

    fn description(&self) -> &'static str {
        "This is Route A"
    }

    async fn run(&self, context: &mut TestRobot) -> Result<(), Box<dyn Error>> {
        println!("RouteA run");
        Ok(())
    }
}

struct RouteB;

#[async_trait]
impl AutonRoutine<TestRobot> for RouteB {
    type Return = Result<(), Box<dyn Error>>;

    fn name(&self) -> &'static str {
        "RouteB"
    }

    fn description(&self) -> &'static str {
        "This is Route B"
    }

    async fn run(&self, context: &mut TestRobot) -> Result<(), Box<dyn Error>> {
        println!("RouteB run");
        Ok(())
    }
}

struct TestRobot {
    imu: Rc<Mutex<InertialSensor>>,
}

impl CompeteWithSelector for TestRobot {
    type Category = Category;
    type Return = Result<(), Box<dyn Error>>;

    async fn driver(&mut self) {
        println!("Driver control");
    }

    fn autonomous_routes<'a, 'b>(
        &'b self,
    ) -> BTreeMap<Category, &'a [&'a dyn AutonRoutine<TestRobot, Return = Self::Return>]>
    where
        Self: 'a,
    {
        let mut map = BTreeMap::<Category, &[&dyn AutonRoutine<_, Return = _>]>::new();
        map.insert(Category::RouteA, &[&RouteA]);
        map.insert(Category::RouteB, &[&RouteB]);
        map
    }

    fn is_gyro_calibrating(&self) -> bool {
        self.imu
            .try_lock()
            .map(|imu| imu.is_calibrating().unwrap_or(false))
            // If we can't lock the mutex, assume it's calibrating
            .unwrap_or(true)
    }

    fn calibrate_gyro(&mut self) {
        let imu = self.imu.clone();
        spawn(async move {
            let mut imu = imu.lock().await;
            if let Err(_) = imu.calibrate().await {
                // Try, try again
                _ = imu.calibrate().await;
            }
        })
        .detach();
    }

    fn diagnostics(&self) -> Vec<(alloc::string::String, alloc::string::String)> {
        alloc::vec![(
            "IMU heading".into(),
            self.imu
                .try_lock()
                .map(|imu| imu.heading().unwrap_or(0.0).to_string())
                .unwrap_or("Unavailable".into()),
        )]
    }
}

#[vexide::main]
async fn main(peripherals: Peripherals) {
    let robot = TestRobot {
        imu: Rc::new(Mutex::new(InertialSensor::new(peripherals.port_20))),
    };
    robot.compete_with_selector(peripherals.display).await;
}
