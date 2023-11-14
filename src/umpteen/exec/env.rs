use std::{collections::HashMap, fmt::Display};

use uuid::Uuid;

use crate::{
    error::MemoryError,
    repr::value::{Object, Value},
};

#[derive(Debug)]
pub enum StackItem {
    Address(usize),
    Value(Value),
}

impl Display for StackItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackItem::Address(addr) => write!(f, "{}", addr),
            StackItem::Value(val) => write!(f, "{}", val),
        }
    }
}

pub type Stack = Vec<StackItem>;

#[derive(Debug, Default)]
pub struct Memory {
    vars: HashMap<String, Option<Value>>,
    env_id: Uuid,
    parent: Option<Uuid>,
}

impl Memory {
    pub fn new(parent: Option<Uuid>) -> Self {
        Memory {
            vars: HashMap::new(),
            env_id: Uuid::new_v4(),
            parent,
        }
    }

    pub fn declare(&mut self, name: &str) -> Result<(), MemoryError> {
        if self.vars.contains_key(name) {
            panic!("variable already declared")
        } else {
            self.vars.insert(name.to_string(), None);
        }

        Ok(())
    }

    pub fn assign(
        &mut self,
        name: &str,
        index: Option<usize>,
        value: Value,
    ) -> Result<(), MemoryError> {
        if self.vars.contains_key(name) {
            if let Some(idx) = index {
                if let Some(Some(Value::Object(obj))) = self.vars.get_mut(name) {
                    if let Object::List(list) = obj.as_mut() {
                        if idx >= list.len() {
                            list.resize(idx + 1, Value::Empty);
                        }
                        list[idx] = value;
                        return Ok(());
                    }
                }
            } else {
                self.vars.insert(name.to_string(), Some(value));
            }
        } else {
            Err(MemoryError::NoSuchVariable(name.to_string()))?
        }

        Ok(())
    }

    pub fn get(&self, name: &str, index: Option<usize>) -> Result<Value, MemoryError> {
        let Some(Some(var)) = self.vars.get(name) else {
            return Err(MemoryError::UninitializedVariableAccess(name.to_owned()));
        };

        if let Some(idx) = index {
            if let Value::Object(obj) = var {
                if let Object::List(list) = obj.as_ref() {
                    return Ok(list[idx].clone());
                }
            } else {
                Err(MemoryError::CannotIndex(name.to_string()))?;
            }
        }

        Ok(var.clone())
    }

    pub fn parent(&self) -> Option<Uuid> {
        self.parent
    }
}

#[derive(Debug)]
pub struct Env {
    scopes: HashMap<Uuid, Memory>,
    glob_key: Uuid,
    current: Option<Uuid>,
}

impl Env {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, name: &str, index: Option<usize>) -> Result<Value, MemoryError> {
        let mut maybe_mem = Some(self.mem());

        while let Some(mem) = maybe_mem {
            if let Ok(val) = mem.get(name, index) {
                return Ok(val);
            } else if let Some(id) = mem.parent {
                maybe_mem = self.retrieve(id);
            } else {
                let mem = self.retrieve(self.glob_key).unwrap();
                return mem.get(name, index);
            }
        }

        Err(MemoryError::NoSuchVariable(name.to_owned()))
    }

    pub fn declare(&mut self, name: &str) -> Result<(), MemoryError> {
        // Always uses current scope
        self.mem_mut().declare(name)
    }

    pub fn assign(
        &mut self,
        name: &str,
        index: Option<usize>,
        value: Value,
    ) -> Result<(), MemoryError> {
        let mut maybe_mem = Some(self.mem_mut());
        while let Some(mem) = maybe_mem.as_deref_mut() {
            if mem.vars.contains_key(name) {
                return mem.assign(name, index, value);
            } else if let Some(id) = mem.parent {
                maybe_mem = self.retrieve_mut(id);
            } else {
                let mem = self.retrieve_mut(self.glob_key).unwrap();
                return mem.assign(name, index, value);
            }
        }


        Err(MemoryError::NoSuchVariable(name.to_owned()))
    }

    pub fn set_current(&mut self, id: Option<Uuid>) -> Option<Uuid> {
        let current = self.current;
        self.current = id;
        current
    }

    pub fn new_enclosed(&mut self) -> Uuid {
        let key = Uuid::new_v4();
        let mem = Memory::new(self.current);

        self.scopes.insert(key, mem);
        key
    }

    fn retrieve(&self, id: Uuid) -> Option<&Memory> {
        self.scopes.get(&id)
    }

    fn retrieve_mut(&mut self, id: Uuid) -> Option<&mut Memory> {
        self.scopes.get_mut(&id)
    }

    fn mem(&self) -> &Memory {
        match self.current {
            Some(id) => self.scopes.get(&id).unwrap(),
            None => self.scopes.get(&self.glob_key).unwrap(),
        }
    }

    fn mem_mut(&mut self) -> &mut Memory {
        match self.current {
            Some(id) => self.scopes.get_mut(&id).unwrap(),
            None => self.scopes.get_mut(&self.glob_key).unwrap(),
        }
    }
}

impl Default for Env {
    fn default() -> Self {
        let globals = Memory::default();
        let glob_key = Uuid::new_v4();
        let scopes = HashMap::from([(glob_key, globals)]);

        Env {
            scopes,
            glob_key,
            current: None,
        }
    }
}
