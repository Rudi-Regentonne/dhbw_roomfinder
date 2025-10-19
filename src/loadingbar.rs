use terminal_size::{terminal_size, Width};
pub struct Loadingbar {
    label: String,
    progress: usize,
    size: usize,
    width: usize,
}

impl Loadingbar {
    pub fn new(label: &str, size: usize) -> Self {
        Loadingbar {
            label: label.to_owned(),
            progress: 0,
            size: size.to_owned(),
            width: terminal_size()
                .map(|(Width(w), _)| w as usize)
                .unwrap_or(20)
                / 2,
        }
    }
    pub fn next(&mut self) {
        self.progress += 1;
        let length = (self.progress) * (self.width - 3) / (self.size);
        print!(
            "\r{}{}[{}>{}]",
            self.label,
            " ".repeat(self.width - self.label.len()),
            "=".repeat(length),
            " ".repeat(self.width - length - 3)
        );

        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }
    pub fn print(&mut self, text: &str) {
        print!("\r\x1b[K"); //delete line
        println!("{}", text);
        let length = (self.progress) * (self.width) / (self.size);
        print!(
            "\r{} [{}{}]",
            self.label,
            "#".repeat(length),
            "-".repeat(self.width - length)
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
