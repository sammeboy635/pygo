use std::collections::HashMap;
use std::collections::HashSet;
use crate::PygoTypes::pygo_instruction::Instruction;

use super::pygo_type::Type;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VariableType {
    Local,
    Global,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Variable {
    pub name: String,
    pub var_type: VariableType,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Function {
    pub name: String,
    // Add any additional information about the function, such as its parameters and return type
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Class {
    pub name: String,
    // Add any additional information about the class, such as its methods, properties, and inheritance
}
#[derive(Debug, Clone)]
pub struct Context {
    imports: HashSet<String>,
    classes: HashMap<String, Class>,
    pub variables: HashMap<String, Type>,
    functions: HashMap<String, Function>,
	pub instruction: Vec<Instruction>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            imports: HashSet::new(),
            classes: HashMap::new(),
            variables: HashMap::new(),
            functions: HashMap::new(),
			instruction: Vec::new(),
        }
    }

    pub fn add_import(&mut self, name: String) {
        self.imports.insert(name);
    }

    pub fn add_class(&mut self, class: Class) {
        self.classes.insert(class.name.clone(), class);
    }

    pub fn add_variable(&mut self, variable: String, val : Type) {
        self.variables.insert(variable, val);
    }

    pub fn add_function(&mut self, function: Function) {
        self.functions.insert(function.name.clone(), function);
    }

    pub fn find_import(&self, name: &str) -> Option<&String> {
        self.imports.get(name)
    }

    pub fn find_class(&self, name: &str) -> Option<&Class> {
        self.classes.get(name)
    }

    pub fn find_variable(&self, name: &str) -> Option<&Type> {
        self.variables.get(name)
    }

    pub fn find_function(&self, name: &str) -> Option<&Function> {
        self.functions.get(name)
    }
}