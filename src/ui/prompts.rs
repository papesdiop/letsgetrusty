
use crate::models::{Epic, Story, Status};
use crate::io_utils::get_user_input;
 
pub struct Prompts {
    pub create_epic: Box<dyn Fn() -> Epic>,
    pub create_story: Box<dyn Fn() -> Story>,
    pub delete_epic: Box<dyn Fn() -> bool>,
    pub delete_story: Box<dyn Fn() -> bool>,
    pub update_status: Box<dyn Fn() -> Option<Status>>
}

impl Prompts {
    pub fn new() -> Self {
        Self { 
            create_epic: Box::new(create_epic_prompt),
            create_story: Box::new(create_story_prompt),
            delete_epic: Box::new(delete_epic_prompt),
            delete_story: Box::new(delete_story_prompt),
            update_status: Box::new(update_status_prompt)
        }
    }
}

fn create_epic_prompt() -> Epic {
    
    println!("-------------------------------");
    println!("Epic name : ");
    let name = get_user_input();
    println!();
    println!("Epic description : ");
    let description = get_user_input();

    Epic { name , description: description, status:Status::Open, stories: vec![] }

}

fn create_story_prompt() -> Story {
    
    println!("-------------------------------");
    println!("Story name : ");
    let name = get_user_input();
    println!();
    println!("Story description : ");
    let description = get_user_input();

    Story { name, description, status: Status::Open }

}

fn delete_epic_prompt() -> bool {
    println!("----------------------------");
    print!("Are you sure you want to delete this epic? All stories in this epic will also be deleted [Y/n]");
    let input = get_user_input();
    if &input == "Y" {
        return true;
    }
    return false;
}

fn delete_story_prompt() -> bool {
    println!("---------------------------");
    print!("Are you sure you want to delete this story? [Y/n]");
    let input = get_user_input();
    if &input == "Y" {
        return true;
    }
    return false;
}

fn update_status_prompt() -> Option<Status> {
    println!("----------------------------");
    println!("New Status (1 - OPEN, 2 - IN-PROGRESS, 3 - RESOLVED, 4 - CLOSED):");
    let input = get_user_input().parse::<u32>();
    match input {
        Ok(1) => Some(Status::Open),
        Ok(2) => Some(Status::InProgress),
        Ok(3) => Some(Status::Resolved),
        Ok(4) => Some(Status::Closed),
        _   => None,

    }
}