use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Label)]
pub struct Score {
    score: u32,
}

#[methods]
impl Score {
    fn new(_owner: &Label) -> Self {
        Score { score: 0 }
    }
    
    #[method]
    fn _ready(&mut self, #[base] _owner: &Label) {
        // godot_print!("score class loaded?");
    }

    #[method]
    fn on_mob_squashed(&mut self, #[base] owner: &Label) {
        self.score += 1;
        owner.set_text(format!("Score {}", self.score));
    }
}
