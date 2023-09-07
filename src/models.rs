use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    NavigateToEpicDetail { epic_id: u32 },
    NavigateToStoryDetail { epic_id: u32, story_id: u32 },
    NavigateToPreviousPage,
    CreateEpic,
    UpdateEpicStatus { epic_id: u32 },
    DeleteEpic { epic_id: u32 },
    CreateStory { epic_id: u32 },
    UpdateStoryStatus { story_id: u32 },
    DeleteStory { epic_id: u32, story_id: u32 },
    Exit,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Status {
    // TODO: add fields (make sure the fields are public)
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct Epic {
    // TODO: add fields (make sure the fields are public)
   pub name: String,
   pub description: String,
   pub status: Status,
   pub stories: Vec<u32>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        // by default the status should be set to open and the stories should be an empty vector
        Self {
            name,
            description,
            status: Status::Open,
            stories: vec![],
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct Story {
    // TODO: add fields (make sure the fields are public)
    pub name: String,
    pub description:String,
    pub status:Status,

}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        // by default the status should be set to open
        Self{
            name,
            description,
            status: Status::Open,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct DBState {
    // This struct represents the entire db state which includes the last_item_id, epics, and stories
    // TODO: add fields (make sure the fields are public)
    pub last_item_id : u32,
    pub epics: HashMap<u32,Epic>,
    pub stories: HashMap<u32, Story>,
}
