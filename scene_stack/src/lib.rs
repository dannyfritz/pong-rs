use std::error::Error;

pub enum StackChange<Intent, SceneType> {
    Add(Box<Scene<Intent, SceneType>>),
    Pop,
    Swap(Box<Scene<Intent, SceneType>>),
}

pub struct SceneStack<Intent, SceneType> {
    pub scenes: Vec<Box<Scene<Intent, SceneType>>>,
}

impl<Intent, SceneType> SceneStack<Intent, SceneType> {
    pub fn new() -> SceneStack<Intent, SceneType> {
        SceneStack { scenes: vec![] }
    }
    pub fn add(&mut self, scene: Box<Scene<Intent, SceneType>>) {
        self.scenes.push(scene);
    }
    pub fn pop(&mut self) -> Option<Box<Scene<Intent, SceneType>>> {
        self.scenes.pop()
    }
    pub fn interpolate(
        &mut self,
        dt: f64,
        intent: &Intent,
    ) -> Result<Option<StackChange<Intent, SceneType>>, Box<Error>> {
        match self.scenes.split_last_mut() {
            Some((last, rest)) => {
                for scene in rest {
                    scene.interpolate(dt, intent)?;
                }
                last.interpolate(dt, intent)
            }
            None => panic!("No scenes to interpolate!"),
        }
    }
}

pub trait Scene<Intent, SceneType> {
    fn interpolate(
        &mut self,
        dt: f64,
        intent: &Intent,
    ) -> Result<Option<StackChange<Intent, SceneType>>, Box<Error>>;
    fn get_type(&self) -> SceneType;
}
