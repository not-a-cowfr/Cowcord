// TODO: jsut make some build script that autogenerates this file

mod home;
pub use home::Home;

mod app;
pub use app::FateDecider;

mod auth;
pub use auth::{Login, Register};

mod not_found;
pub use not_found::PageNotFound;
