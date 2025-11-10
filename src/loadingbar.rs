use terminal_size::{terminal_size, Width};
pub struct Loadingbar {
    label: String,
    progress: usize,
    size: usize,
    width: usize,
}

impl Loadingbar {
    pub fn new(label: &str, size: usize) -> Self {
        let loadingbar = Loadingbar {
            label: label.to_owned(),
            progress: 0,
            size: size.to_owned(),
            width: terminal_size()
                .map(|(Width(w), _)| w as usize)
                .unwrap_or(20)
                / 2,
        };

        return loadingbar;
    }
    pub fn next(&mut self) {
        self.progress += 1;
        self.print_bar();
    }
    pub fn print(&mut self, text: &str) {
        println!("\r\r\x1b[K{}", text);

        self.print_bar();
    }
    fn print_bar(&self) {
        let length = (self.progress) * (self.width - 3) / (self.size);
        print!(
            "\r\r\x1b[K{}{}[{}>{}]", // delete line
            self.label,
            " ".repeat(self.width - self.label.len()),
            "=".repeat(length),
            " ".repeat(self.width - length - 3)
        );
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }
}

#[macro_export]
macro_rules! loadingbar_println {
($($arg:tt)*) => {
    print!("\r\x1b[K");//delete line
    println!($($arg)*);

};
}
