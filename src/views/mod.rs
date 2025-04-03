// TODO: jsut make some build script that autogenerates this file

mod home;
pub use home::Home;

mod app;
pub use app::FateDecider;

mod auth;
pub use auth::{Login, Register};

mod not_found;
pub use not_found::PageNotFound;

mod chats;
pub use chats::server::channel::channel::Channel;
pub use chats::server::channel::channels_and_roles::Roles;
pub use chats::server::server::Server;
