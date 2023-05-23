use anyhow::Result;
use enigo::{Enigo, Key, KeyboardControllable};
use tao::accelerator::Accelerator;
use tao::clipboard::Clipboard;
use tao::event::Event;
use tao::event_loop::{ControlFlow, EventLoop};
use tao::global_shortcut::ShortcutManager;
use tao::keyboard::KeyCode;
use tao::platform::run_return::EventLoopExtRunReturn;

fn main() -> Result<()> {
    let mut event_loop = EventLoop::new();
    let mut hotkey_manager = ShortcutManager::new(&event_loop);
    let accelerator = Accelerator::new(None, KeyCode::Insert);
    let _global_shortcut = hotkey_manager.register(accelerator.clone())?;
    let clipboard = Clipboard::new();
    let mut enigo = Enigo::new();

    event_loop.run_return(move |event, _, flow| {
        *flow = ControlFlow::Wait;
        match event {
            Event::GlobalShortcutEvent(id) if id == accelerator.clone().id() => {
                if let Some(text) = clipboard.read_text() {
                    let mut first = true;
                    for line in text.lines() {
                        if !first {
                            enigo.key_click(Key::Return);
                        }
                        enigo.key_sequence(line);
                        first = false;
                    }
                }
            }
            _ => {}
        }
    });
    Ok(())
}
