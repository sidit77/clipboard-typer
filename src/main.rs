mod error;
mod clipboard;

use crate::clipboard::Clipboard;
use crate::error::WinResult;

fn main() -> WinResult<()> {
    println!("{}", Clipboard::new()?.get_text()?);
    Ok(())
}
