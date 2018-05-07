use std::error::Error;

pub enum SceneChange<Intents, SceneType> {
    Add(Box<Scene<Intents, SceneType>>),
    Pop,
    Swap(Box<Scene<Intents, SceneType>>),
}

pub struct SceneStack<Intents, SceneType> {
    pub scenes: Vec<Box<Scene<Intents, SceneType>>>,
}

impl<Intents, SceneType> SceneStack<Intents, SceneType> {
    pub fn new() -> SceneStack<Intents, SceneType> {
        SceneStack { scenes: vec![] }
    }
    pub fn add(&mut self, scene: Box<Scene<Intents, SceneType>>) {
        self.scenes.push(scene);
    }
    pub fn pop(&mut self) -> Option<Box<Scene<Intents, SceneType>>> {
        self.scenes.pop()
    }
    pub fn interpolate(
        &mut self,
        dt: f64,
        intents: &Intents,
    ) -> Result<Option<SceneChange<Intents, SceneType>>, Box<Error>> {
        match self.scenes.split_last_mut() {
            Some((last, rest)) => {
                for scene in rest {
                    scene.interpolate(dt, intents)?;
                }
                last.interpolate(dt, intents)
            }
            None => panic!("No scenes to interpolate!"),
        }
    }
}

pub trait Scene<Intents, SceneType> {
    fn interpolate(
        &mut self,
        dt: f64,
        intents: &Intents,
    ) -> Result<Option<SceneChange<Intents, SceneType>>, Box<Error>>;
    fn get_type(&self) -> SceneType;
}
