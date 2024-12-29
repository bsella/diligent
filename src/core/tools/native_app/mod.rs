mod command_line_parser;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub fn main() -> Result<(), std::io::Error> {
    linux::main()
}
