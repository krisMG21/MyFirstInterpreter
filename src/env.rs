use std::collections::HashMap;
use crate::val::Val;

#[derive(Debug, PartialEq, Default)]
pub(crate) struct Env{
    bindings: HashMap<String, Val>,
}

impl Env{
    pub(crate) fn store_bindings(&mut self, name:String, value:Val){
        self.bindings.insert(name,value);
    }
}