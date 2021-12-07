use rusty_v8 as v8;

use std::collections::HashMap;

type ModuleId = v8::Global<v8::Module>;

pub struct ModuleInfos {
    pub filepath: String,
}

pub struct ModuleMap {
    infos: HashMap<ModuleId, ModuleInfos>,
}

impl ModuleMap {
    pub fn new() -> ModuleMap {
        ModuleMap {
            infos: HashMap::new(),
        }
    }

    pub fn insert(&mut self, global: v8::Global<v8::Module>, infos: ModuleInfos) {
        self.infos.insert(global, infos);
    }

    pub fn get(&self, global: &v8::Global<v8::Module>) -> Option<&ModuleInfos> {
        self.infos.get(global)
    }

    pub fn clear(&mut self) {
        self.infos.clear();
    }

    pub fn find_by_filepath(
        &self,
        filepath: &str,
    ) -> Option<(&v8::Global<v8::Module>, &ModuleInfos)> {
        self.infos.iter().find(|pair| pair.1.filepath == filepath)
    }
}
