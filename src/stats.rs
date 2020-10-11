use gdnative::prelude::{ClassBuilder, NativeClass, Node};

#[derive(NativeClass)]
#[inherit(Node)]
#[derive(Default)]
#[register_with(register_properties)]
pub struct Stats {
    pub(crate) max_health: i32,
    pub(crate) health: i32,
}

fn register_properties(builder: &ClassBuilder<Stats>) {
    builder
        .add_property::<i32>("max_health")
        .with_getter(|stats: &Stats, _| stats.max_health)
        .with_setter(|stats: &mut Stats, _, max_health| stats.max_health = max_health)
        .done();
}

#[gdnative::methods]
impl Stats {
    fn new(_owner: &Node) -> Self {
        let max_health = 1;
        Self {
            max_health,
            health: max_health,
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node) {
        self.health = self.max_health
    }

    pub fn receive_damage(&mut self, amount: i32) -> State {
        self.health -= amount;

        if self.health <= 0 {
            return State::Dead;
        }

        return State::Alive;
    }
}

pub enum State {
    Alive,
    Dead,
}
