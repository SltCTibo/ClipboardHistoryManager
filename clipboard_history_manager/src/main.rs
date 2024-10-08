use copypasta::{ClipboardContext, ClipboardProvider};

fn main() {
    let mut clipboard = ClipboardContext::new().unwrap();

    let clipboard_content = clipboard.get_contents().unwrap_or_else(|_| String::from("The clipboard is empty or there is no access"));

    println!("Actual clipboard content: {}", clipboard_content);
}
