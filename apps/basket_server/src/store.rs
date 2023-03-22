use basket_defs::{command::{Key, Value, Command, Action}, command_result::CommandResult};
use hashbrown::HashMap;

pub struct Store {
    map: HashMap<Key, Value>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn apply(&mut self, command: &Command) -> CommandResult {
        match &command.action {
            Action::SET(key, value) => {
                self.map.insert(*key, *value);
                CommandResult::Ok
            },

            Action::GET(key) => {
                match self.map.get(key) {
                    Some(v) => CommandResult::Value(*v),
                    None => CommandResult::NotFound
                }
            },
            
            Action::DEL(key) => {
                match self.map.remove(key) {
                    Some(_) => CommandResult::Ok,
                    None => CommandResult::NotFound
                }
            }
        }
    }
}
