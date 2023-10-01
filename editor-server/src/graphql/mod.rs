pub(crate) mod types;
pub(crate) mod schema;
pub(crate) mod subscriptor;

mod mutations;
mod queries;
mod subscriptions;

use mutations::*;
use queries::*;
use subscriptions::*;
