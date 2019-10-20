#[cfg(target_os = "windows")]
#[path = "platform/windows/mod.rs"]
pub mod window;

#[cfg(target_os = "linux")]
#[path = "platform/linux/mod.rs"]
pub mod window;

#[cfg(target_os = "macos")]
#[path = "platform/macos/mod.rs"]
pub mod window;

#[cfg(all(
    not(target_os = "windows"),
    not(target_os = "linux"),
    not(target_os = "macos"),
))]
compile_error!("The platform you're compiling for is currently not supported.");

pub fn print_me(word: &str) {
    println!("The word is: {}.", word);
}