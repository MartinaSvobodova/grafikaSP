use super::model_group::ModelGroup;

#[derive(Clone, Debug, Default)]
pub struct Scene {
    pub groups: Vec<ModelGroup>,
}

impl Scene {
    pub fn new() -> Self {
        Self { groups: Vec::new() }
    }

    pub fn with_groups(groups: Vec<ModelGroup>) -> Self {
        Self { groups }
    }

    pub fn add_group(&mut self, group: ModelGroup) {
        self.groups.push(group);
    }
}
