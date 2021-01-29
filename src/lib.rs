pub mod db;
pub mod error;
pub mod package;
pub mod parser;

/// All essential symbols in a simple consumable way
///
/// ### Examples
/// ```
/// use package_iter::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{db::*, error::*, package::*};
}
