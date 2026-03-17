use super::model::Model;

#[derive(Clone, Debug)]
pub struct ModelGroup {
    pub name: String,
    pub models: Vec<Model>,
}

impl ModelGroup {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            models: Vec::new(),
        }
    }

    pub fn with_models(name: impl Into<String>, models: Vec<Model>) -> Self {
        Self {
            name: name.into(),
            models,
        }
    }

    pub fn add_model(&mut self, model: Model) {
        self.models.push(model);
    }
}
