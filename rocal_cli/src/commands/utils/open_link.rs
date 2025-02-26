use std::io;
use std::process::{Command, Stdio};

#[cfg(target_os = "macos")]
pub fn open_link(url: &str) -> io::Result<()> {
    Command::new("open")
        .arg(url)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn open_link(url: &str) -> io::Result<()> {
    Command::new("xdg-open")
        .arg(url)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn open_link(url: &str) -> io::Result<()> {
    Command::new("cmd")
        .args(&["/C", "start", "", url])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    Ok(())
}
