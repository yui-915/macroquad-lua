use super::*;

wrap_functions_for_lua! {
    pub wrap mq::rand::rand as rand
        () -> u32

    pub wrap mq::rand::srand as srand
        (seed: u64) -> ()
}
