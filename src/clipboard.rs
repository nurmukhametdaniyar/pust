use copypasta::{ClipboardContext, ClipboardProvider};

pub struct Clipboard;

impl Clipboard {
    pub fn copy_text(text: String) {
        let mut ctx = ClipboardContext::new().unwrap();
        ctx.set_contents(text.to_owned()).unwrap();
    }
}
