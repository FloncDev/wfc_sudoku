pub mod sudoku;

fn main() {
    tracing_subscriber::fmt().init();

    tracing::info!("Hello, world!");
}
