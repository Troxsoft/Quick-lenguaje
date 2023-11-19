use std::fs::read;

use crate::leng::{error_leng::ErrorLeng, qlenguaje::Programa};
#[derive(Clone)]
pub struct Funciones {
    funciones: Vec<fn(Vec<String>, Programa) -> Result<Programa, ErrorLeng>>,
    nombre_funciones: Vec<String>,
    prefix: String,
}

impl Funciones {
    pub fn new(prefix: String) -> Self {
        Funciones {
            funciones: Vec::new(),
            nombre_funciones: Vec::new(),
            prefix: prefix,
        }
    }

    pub fn add_funcion(
        &mut self,
        func: fn(Vec<String>, Programa) -> Result<Programa, ErrorLeng>,
        nombre: String,
    ) {
        self.funciones.push(func);
        self.nombre_funciones.push(nombre);
    }
    pub fn funcion(
        &self,
        nombre: String,
    ) -> Option<fn(Vec<String>, Programa) -> Result<Programa, ErrorLeng>> {
        let mut i = 0;
        while i < self.funciones.len() {
            if self.nombre_funciones[i].eq(&nombre) {
                return Some(self.funciones[i]);
            }
            i += 1;
        }
        return None;
    }
    pub fn is_prefix(&self, g: String) -> bool {
        if self.prefix == g {
            return true;
        } else {
            return false;
        }
    }
    pub fn prefix(&self) -> String {
        self.prefix.to_string()
    }
}
