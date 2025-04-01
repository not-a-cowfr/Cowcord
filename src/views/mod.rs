// TODO: jsut make some build script that autogenerates this file

mod home;
pub use home::Home;

mod auth;
pub use auth::{Login, Register};

mod chats;
pub use chats::server::channel::channel::Channel;
pub use chats::server::channel::channels_and_roles::Roles;
pub use chats::server::server::Server;
