#[derive(Clone)]
pub struct InputPin {
    value : usize,
    bound_register_id: usize,
}

impl InputPin {
    pub fn get_value(&self) -> usize {
        self.value
    }

    pub fn get_bound_register_id(&self) -> usize {
        self.bound_register_id
    }
}

#[derive(Clone)]
pub struct OutputPin {
    value : usize,
    bound_register_id: usize,
}

impl OutputPin {
    pub fn get_value(&self) -> usize {
        self.value
    }

    pub fn get_bound_register_id(&self) -> usize {
        self.bound_register_id
    }
}