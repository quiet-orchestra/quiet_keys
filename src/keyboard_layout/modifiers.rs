use std::collections::HashMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeMap;
use serde::de::Error;

//Modifier Structs
const MODIFIER_COUNT: usize = 8;

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct ModifierMask(u8);

impl ModifierMask {
    pub const fn base() -> Self{
        return Self(0b0000_0000);
    }
    pub const fn shift() -> Self{
        return Self(0b0000_0001);
    }
    pub const fn alt_gr() -> Self{
        return Self(0b0000_0010);
    }
    pub const fn shift_alt_gr() -> Self{
        return Self(0b0000_0011);
    }

    pub fn get_indices_from_mask(&self) -> Vec<ModifierIndex>{
        let mut index_vec = Vec::new();
        for index in 0..MODIFIER_COUNT {
            if self.0 >> index & 1 == 1 {
                index_vec.push(ModifierIndex(index));
            }
        }
        return index_vec;
    }

}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]//Need to validate the usize as less that MODIFIER_COUNT during deserialization.
pub struct ModifierIndex(usize);
impl ModifierIndex {
    pub fn from_usize(index: usize) -> Option<Self> {
        if index < MODIFIER_COUNT {
            Some(ModifierIndex(index))
        }
        else{
            None
        }
    }
    pub fn to_mask(&self) -> ModifierMask{
        ModifierMask(1u8 << self.0)
    }
    pub const fn shift() -> Self{
        return Self(0);
    }
    pub const fn alt_gr() -> Self{
        return Self(1);
    }
}

#[derive(Clone, Debug, Default)]
pub struct ModifierList{modifier_list: [Option<String>;MODIFIER_COUNT]}

impl ModifierList {

    pub fn get_index(&self, requested_modifier: &str) -> Option<ModifierIndex>{
        let modifier_index = None;
        for index in 0..MODIFIER_COUNT {
            if let Some(modifier) = &self.modifier_list[index] {
                if modifier.eq(requested_modifier) {
                    return Some(ModifierIndex(index));
                }
            }
        }
        return modifier_index;
    }

    pub fn get_mask(&self) -> ModifierMask{
        let ModifierMask(mut mask) = ModifierMask::base();
        for index in 0..MODIFIER_COUNT {
            if self.modifier_list[index].is_some() {
                mask |= 1 << index;
            }
        }
        return ModifierMask(mask);
    }

    fn get_modifier(&self, index: ModifierIndex) -> Option<String> {
        return self.modifier_list[index.0].clone();
    }

    pub fn get_modifiers_and_indices(&self) -> Vec<(String, ModifierIndex)> {
        let mask = self.get_mask();
        let indices_vec: Vec<ModifierIndex> = mask.get_indices_from_mask();
        indices_vec.iter().map(|index| {
            let modifier = self.get_modifier(*index).expect("Modifier should be guranteed from mask.");//Is verified to be there.
            (modifier,*index)
        }).collect()
    }

    pub fn register_modifier_with_index(&mut self, modifier: impl Into<String>, index: ModifierIndex){
        self.modifier_list[index.0] = Some(modifier.into());    
    }

    pub fn register_shift(&mut self){
        self.register_modifier_with_index("Shift", ModifierIndex::shift());
    }
    pub fn register_alt_gr(&mut self){
        self.register_modifier_with_index("AltGr", ModifierIndex::alt_gr());
    }

    pub fn register_modifier(&mut self, modifier: impl Into<String>) -> bool{
        let modifier = modifier.into();
        if self.get_index(&modifier).is_some(){true}
        else{
            if let Some(index) = self.get_first_empty_index() {
                self.register_modifier_with_index(modifier, index);
                true
            }
            else {false}
        }
    }

    fn get_first_empty_index(&self) -> Option<ModifierIndex> {
        for index in 0..MODIFIER_COUNT{
            if self.modifier_list[index].is_none() {
                return Some(ModifierIndex(index));
            }
        }
        return None;
    }
}

impl Serialize for ModifierList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let modifier_list =  self.get_modifiers_and_indices();
        let mut map = serializer.serialize_map(Some(modifier_list.len()))?;
        for (modifier, index) in modifier_list {
            map.serialize_entry(&index.0, &modifier)?;
        } 
        map.end()
    }
}
impl<'de> Deserialize<'de> for ModifierList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut modifier_list = [const { None };MODIFIER_COUNT];
        let map: HashMap<usize,String> = HashMap::deserialize(deserializer)?;
        for (index, modifier) in map{
            if index < MODIFIER_COUNT {
                modifier_list[index] = Some(modifier);
            }
            else{
                return Err(D::Error::custom(format!(
                    "Given modifier index {index} is out of range; valid indices are between 0 and {}",
                    MODIFIER_COUNT - 1
                )));
            }
        }
        Ok(ModifierList{modifier_list})
    }
}

pub fn is_base_modifier_mask(mask: &ModifierMask) -> bool {
    mask.0 == 0
}