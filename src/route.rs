use std::{
    fmt::{Debug, Display},
    future::Future,
    pin::Pin,
};

pub trait Category = Clone + Copy + Eq + Debug + Display + Ord;
type RouteFn<Shared> = for<'s> fn(&'s mut Shared) -> Pin<Box<dyn Future<Output = ()> + 's>>;

/// Route entry for [`DoxaSelect`].
///
/// These are provided to [`DoxaSelect`] in the form of an array passed to [`DoxaSelect`].
/// Route entries contain a function pointer to the provided route function, as well as a human-readable
/// name for the route that is displayed in the selector's UI.
///
/// It's recommended to use the [`route!()`] macro to aid in creating instances of this struct.
///
/// [`DoxaSelect`]: crate::DoxaSelect
#[derive(Debug)]
pub struct Route<C: Category, R> {
    pub category: C,
    pub name: &'static str,
    pub description: &'static str,
    pub callback: RouteFn<R>,
}

impl<C: Category, R> Clone for Route<C, R> {
    fn clone(&self) -> Self {
        Self {
            category: self.category,
            name: self.name,
            description: self.description,
            callback: self.callback,
        }
    }
}

/// Concisely creates an instance of a [`Route`].
///
/// # Example
///
/// ```ignore
/// let routes = [
///     route!("Route 1", "A test route.", Robot::route_1),
///     route!("Route 2", "Another test route.", Robot::route_2),
/// ];
/// ```
#[macro_export]
macro_rules! route {
    ($category:expr, $func:path) => {{
        ::doxa_selector::Route {
            category: $category,
            name: stringify!($func),
            description: "",
            callback: |robot| ::std::boxed::Box::pin($func(robot)),
        }
    }};
    ($category:expr, $name:expr, $func:path) => {{
        ::doxa_selector::Route {
            category: $category,
            name: $name,
            description: "",
            callback: |robot| ::std::boxed::Box::pin($func(robot)),
        }
    }};
    ($category:expr, $name:expr, $description:expr, $func:path) => {{
        ::doxa_selector::Route {
            category: $category,
            name: $name,
            description: $description,
            callback: |robot| ::std::boxed::Box::pin($func(robot)),
        }
    }};
}
pub use route;
