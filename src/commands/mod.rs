pub mod list;
pub mod logs;
pub mod setup;

pub use setup::*;
pub use logs::{fetch_logs};
pub use list::{list_pods, list_deployments, list_statefulsets};