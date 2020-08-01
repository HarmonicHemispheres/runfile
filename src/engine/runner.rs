
use uuid::Uuid;
use crate::parser::actions::Action;
use crate::engine::settings::EngineSettings;
use crate::engine::engine::Engine;


#[derive(Debug, Clone)]
pub struct Runner {
    hash_id: String,
    actions: Vec<Action>,
    engine: Engine,
    action_idx: usize
}


impl Iterator for Runner {
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        if self.action_idx <= self.actions.len()-1 {
            let idx_to_return = self.action_idx;
            self.action_idx += 1;
            Some(self.actions[idx_to_return].clone())

        } else {
            None
        }
    }
}

impl Runner {
    pub fn new(actions: Vec<Action>, settings: EngineSettings) -> Runner 
    {
        let hash = Uuid::new_v4();
        Runner {
            hash_id: hash.to_simple().to_string(),
            actions: actions,
            engine: Engine::new(settings),
            action_idx: 0
        }
    }


    pub fn run(&mut self){
        // run commands via engine


        for action in self {
            println!("{:?}", action);
        }
    }
}