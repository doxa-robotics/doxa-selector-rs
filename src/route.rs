use std::{future::Future, pin::Pin};

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
pub struct Route<R> {
    pub name: &'static str,
    pub description: &'static str,
    pub callback: RouteFn<R>,
}

impl<R> Clone for Route<R> {
    fn clone(&self) -> Self {
        Self {
            name: self.name,
            description: self.description,
            callback: self.callback,
        }
    }
}

impl<R> Route<R> {
    pub const fn new(name: &'static str, description: &'static str, callback: RouteFn<R>) -> Self {
        Self {
            name,
            description,
            callback,
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
    ($func:path) => {{
        ::doxa_selector::Route::new(stringify!($func), "", |robot| {
            ::std::boxed::Box::pin($func(robot))
        })
    }};
    ($name:expr, $func:path) => {{
        ::doxa_selector::Route::new($name, "", |robot| ::std::boxed::Box::pin($func(robot)))
    }};
    ($name:expr, $description:expr, $func:path) => {{
        ::doxa_selector::Route::new($name, $description, |robot| {
            ::std::boxed::Box::pin($func(robot))
        })
    }};
}
pub use route;
