mod models;
mod db;
mod ui;
mod io_utils;
mod navigator;

fn main() {
    let s = ui::page_helpers::get_column_string("testme",6);
    println!("Hello, world!{}",s);
}
