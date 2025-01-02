use crate::types::Environment;

#[derive(Debug)]
pub struct InMemoryStorage {
    pub data: Vec<Environment>,
}

impl InMemoryStorage {
    fn store_environment(&mut self, environment: Environment) {
        println!("Storing environment: {:?}", environment);
        self.data.push(environment);
    }

    /* fn get_environments(&self) -> Vec<Environment> {
        self.data.clone()
    } */
}
