macro_rules! clear {
    () => {print!("{esc}c", esc = 27 as char)}
}
macro_rules! press_enter_to_continue {
    () => {
        printc!("\nPress enter to continue ...", WHITE);
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut String::new()).unwrap();
        clear!();
    }
}
macro_rules! continue_or_return {
    () => {
        printc!("Press enter to continue or C to return to menu ...", WHITE);
        let mut res = String::new();
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut res).unwrap();
        match res.chars().next().unwrap() {
            'c' => break,
            _ => crossterm_cursor::cursor().move_up(1).unwrap()
        }
    }
}
macro_rules! printc {
    ($text:expr, $color:expr) => {
        let mut t = term::stdout().unwrap();
        t.fg($color).unwrap();
        t.write($text.as_bytes()).unwrap();
    };
}
macro_rules! printlnc {
    ($text:expr, $color:expr) => {
        let mut t = term::stdout().unwrap();
        t.fg($color).unwrap();
        t.write(format!("{}\n", $text).as_bytes()).unwrap();
    };
}
macro_rules! print_title {
    ($title:expr) => {
        clear!();
        printlnc!(format!("- {} -\n", $title), term::color::GREEN);
    };
}