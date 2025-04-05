use std::any::Any;

pub trait ConfigFile: Send + Sync {
    fn load_or_create(filepath: &str) -> Result<Self, String> where Self: Sized;
    fn save(&self, filepath: &str);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}


#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum ConfigType {
    Settings,
    SaveData,
}