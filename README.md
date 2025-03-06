# doxa-selector-rs

A simple autonomous selector written for the `vexide` platform. Its usage is simple and aligns with the `Compete` trait provided by `vexide-core`, except with a little twist.

This library has not been updated to use the `autons` crate but probably will be eventually once I find time.

> [!NOTE]
> The branding of the crate is currently set to our team's branding. However, switching the assets is simple -- just fork the repo and replace the assets folder. (We don't have configurable branding because Slint isn't flexible enough for that in terms of static image inclusion.)
>
> ~~Slint may have not been the right choice for this project as it's a little bit too bloated for this use case.~~ vexide support for patch uploads has fixed this!

```rust
struct TestRobot {
    // ...
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
}

#[vexide::main]
async fn main(peripherals: Peripherals) {
    let robot = TestRobot {
    };
    robot.compete_with_selector(peripherals.display).await;
}
```

You provide a map of autonomous routes, and this library handles rendering the rest.
