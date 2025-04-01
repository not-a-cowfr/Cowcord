// TODO: jsut make some build script that autogenerates this file

mod home;
pub use home::Home;

mod auth;
pub use auth::{Login, Register};

mod server;
pub use server::channel::channel::Channel;
pub use server::channel::channels_and_roles::Roles;
pub use server::server::Server;
