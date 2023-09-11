use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use models::Action;
use navigator::*;

fn main() {
    // TODO: create database and navigator
    let db = JiraDatabase::new("data/db.json".to_owned());
    let mut navigator = Navigator::new(Rc::new(db));

    loop {
        clearscreen::clear().expect("failed to clear screen");
        let page = navigator.get_current_page();
        if let Some(page) = page {
            if let Err(error) = page.draw_page() {
                println!("Error rendering page: {}\nPress any key to continue...", error);
                wait_for_key_press();
                break;
            }
            let input = get_user_input();
            let action = page.handle_input(&input);
            if let Ok(action) = action {
                if let Some(action) = action{
                    if action == Action::Exit {
                        break;
                    }
                    let _ = navigator.handle_action(action);
                }
            }
        } else {
            break;
        }
        // TODO: implement the following functionality:
        // 1. get current page from navigator. If there is no current page exit the loop.
        // 2. render page
        // 3. get user input
        // 4. pass input to page's input handler
        // 5. if the page's input handler returns an action let the navigator process the action
    }
}