pub mod handler;
pub mod registry;

pub use handler::{Handler, HandlerContext, HandlerResult};
pub use registry::{HandlerRegistry, register_default_handlers}; 