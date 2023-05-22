mod clipboard;
mod error;
mod messages;

use crate::clipboard::Clipboard;
use crate::error::WinResult;
use crate::messages::{run_event_loop, Event};

fn main() -> WinResult<()> {
    //println!("{}", Clipboard::new()?.get_text()?);
    run_event_loop(|event| match event {
        Event::Hotkey => println!("Hotkey")
    })?;
    Ok(())
}
