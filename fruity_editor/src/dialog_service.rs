use fruity_core::resource::resource::Resource;

pub trait DialogService: Resource {
    fn save(&self, default_filename: &str, file_types: &[&str]) -> Option<String>;
    fn open(&self, file_types: &[&str]) -> Option<String>;
}
