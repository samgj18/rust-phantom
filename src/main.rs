use std::marker::PhantomData;

struct Locked;

struct Unlocked;

struct StatefulManager<S = Locked> {
    state: PhantomData<S>,
}

impl StatefulManager<Locked> {
    pub fn unlock(self) -> StatefulManager<Unlocked> {
        StatefulManager { state: PhantomData }
    }
}

impl StatefulManager<Unlocked> {
    pub fn lock(self) -> StatefulManager<Locked> {
        StatefulManager { state: PhantomData }
    }

    pub fn do_something(&self) {
        println!("Doing something");
    }
}

impl<State> StatefulManager<State> {
    pub fn do_something_regardless_of_state(&self) {
        println!("Doing something regardless of state");
    }
}

impl StatefulManager {
    pub fn new() -> Self {
        StatefulManager { state: PhantomData }
    }
}

fn main() {
    let manager = StatefulManager::new();
    let unlocked_manager = manager.unlock();
    unlocked_manager.do_something();
    unlocked_manager.do_something_regardless_of_state();
    let locked_manager = unlocked_manager.lock();
    locked_manager.do_something_regardless_of_state();

}
