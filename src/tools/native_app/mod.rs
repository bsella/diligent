use app::App;
pub mod app;

pub mod events;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub fn main<Application>() -> Result<(), std::io::Error>
where
    Application: App,
{
    linux::main::<Application>()
}
