use terminal_size::{terminal_size, Width};
/// Loadingbar shows a progress bar in the terminal with a customizable label, width and progress tracking.
pub struct Loadingbar {
    label: String,
    progress: usize,
    size: usize,
    width: usize,
}

impl Loadingbar {
    /// Creates a new Loadingbar with the given label and total size (number of steps).
    /// The width is set based on the terminal size.
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

    /// Advances the progress bar by one step and prints the updated bar.
    pub fn next(&mut self) {
        self.progress += 1;
        self.print_bar();
    }

    /// Prints a message above the loading bar, then reprints the updated bar.
    pub fn print(&mut self, text: &str) {
        println!("\r\r\x1b[K{}", text);

        self.print_bar();
    }

    /// Renders the loading bar with its current progress and label.
    fn print_bar(&self) {
        let length = (self.progress) * (self.width - 3) / (self.size);
        print!(
            "\r\r\x1b[K{}{}[{}>{}]",
            self.label,
            " ".repeat(self.width - self.label.len()),
            "=".repeat(length),
            " ".repeat(self.width - length - 3)
        );
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }
}
