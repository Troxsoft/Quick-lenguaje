use core::num;
use std::{collections::btree_map::VacantEntry, fs::read};

use super::error_leng::{self, ErrorLeng};
use crate::{
    funciones::{self, fstructure::Funciones},
    utils,
};
#[derive(Clone)]
pub struct Programa {
    funciones: Vec<Funciones>,
    pub detener: bool,
    pub salir: bool,
    variables: Vec<String>,
    variables_nombres: Vec<String>,
    variables_vectores: Vec<Vec<String>>,
    variables_vectores_nombres: Vec<String>,
}

impl Programa {
    pub fn new() -> Self {
        Programa {
            funciones: Vec::new(),
            detener: false,
            salir: false,
            variables: Vec::new(),
            variables_nombres: Vec::new(),
            variables_vectores: Vec::new(),
            variables_vectores_nombres: Vec::new(),
        }
    }
    pub fn get_variables_all_nombre(&self) -> Vec<String> {
        return self.variables_nombres.clone();
    }
    pub fn get_variable_all_valor(&self) -> Vec<String> {
        return self.variables.clone();
    }
    pub fn get_for_prefix(&self, prefix: String) -> Option<Funciones> {
        let ko33 = self.funciones.clone();

        for lo in ko33 {
            if lo.is_prefix(prefix.clone()) {
                return Some(lo);
            }
        }
        None
    }
    pub fn add_variable(&mut self, nombre: String, valor: String) {
        self.variables.push(valor.trim().to_string());
        self.variables_nombres.push(nombre);
    }
    pub fn get_variable(&mut self, nombre: String) -> Option<String> {
        let mut i = 0;
        while i < self.variables_nombres.len() {
            if self.variables_nombres[0] == nombre {
                return Some(self.variables[0].clone());
            }
            i += 1;
        }
        return None;
    }
    pub fn set_variable(&mut self, programa: Programa, nombre: String, valor: String) -> ErrorLeng {
        let mut i = 0;
        while i < self.get_variables_all_nombre().clone().len() {
            if self.get_variables_all_nombre().clone()[i] == nombre {
                self.variables[i] = valor.clone();
                return ErrorLeng::new_ignore(programa);
            }
            i += 1;
        }
        return ErrorLeng::new("no se encontro la variable".to_string(), programa, false);
    }
    pub fn set_variable2(&mut self, nombre: String, valor: String) -> ErrorLeng {
        let mut i = 0;
        while i < self.get_variables_all_nombre().clone().len() {
            if self.get_variables_all_nombre().clone()[i] == nombre {
                self.variables[i] = valor.clone();
                return ErrorLeng::new_ignore(self.clone());
            }

            i += 1;
        }
        return ErrorLeng::new(
            "no se encontro la variable".to_string(),
            self.clone(),
            false,
        );
    }
    //<vec> $·$·$·$:juan$·$·$·$:juan$·$·$·$:
    //vectores / arrays
    pub fn get_vector_variable(&mut self, nombre: String) -> Option<Vec<String>> {
        let mut i: usize = 0;
        while i < self.variables_vectores_nombres.len() {
            if self.variables_vectores_nombres.clone()[i] == nombre {
                return Some(self.variables_vectores[1].clone());
            }
            i += 1;
        }
        None
    }

    pub fn get_vector_variable_all_valor(&mut self) -> Vec<Vec<String>> {
        return self.variables_vectores.clone();
    }
    pub fn get_vector_variable_all_nombre(&mut self) -> Vec<String> {
        return self.variables_vectores_nombres.clone();
    }
    pub fn add_to_vector_element(&mut self, nombre: String, valor: String) -> ErrorLeng {
        let mut i: usize = 0;
        while i < self.variables_vectores_nombres.len() {
            if self.variables_vectores_nombres.clone()[i] == nombre {
                self.variables_vectores[i].push(valor);
                return ErrorLeng::new_ignore(self.clone());
            }
            i += 1;
        }
        return ErrorLeng::new(
            "no se encontro la variable".to_string(),
            self.clone(),
            false,
        );
    }
    pub fn create_add_vector_element(&mut self, nombre: String) {
        self.variables_vectores.push(Vec::new());
        self.variables_vectores_nombres.push(nombre);
    }
    pub fn add_funcion(&mut self, f: Funciones) {
        self.funciones.push(f);
    }

    pub fn init_program(mut programa: Programa, codigo: String) -> Option<ErrorLeng> {
        let mut lineas = codigo.lines();

        //let mut error_mas_reciente: ErrorLeng = ErrorLeng::new_ignore(programa2.clone());
        for mut linea in lineas {
            let mut programa2 = programa.clone();
            let mut programa3 = programa2.clone();
            let mut programa4 = programa3.clone();
            let mut programa5 = programa4.clone();
            let mut programa6: Programa = programa5.clone();
            //preprosesado

            linea = linea.trim();
            if linea != "" && !linea.starts_with("--") {
                let mut linea_String = linea.to_string();

                if programa6.clone().salir {
                    return None;
                }
                if !linea_String.starts_with("bucle") {
                    // VARIABLES
                    for kok in programa.get_variables_all_nombre() {
                        linea_String = linea_String.replace(
                            &format!("@[{}]", kok),
                            &programa.get_variable(kok.clone()).unwrap().clone(),
                        );

                        //println!("'{}'", &programa.get_variable(kok.clone()).unwrap().clone());
                    }

                    // MACROS
                    if linea_String.contains("#+#") {
                        let mut jb: bool = linea_String.contains("#+#");
                        //println!("'{}'", "j");
                        while jb == true {
                            //println!("{}", linea_String);
                            //let index_macro_init = linea_String.find("#+#").unwrap();
                            let mut hfj = linea_String.split(" ");
                            let mut hfj_vec: Vec<String> = Vec::new();
                            let mut index_macro: usize = 0;
                            let mut i91: usize = 0;
                            let mut ya: bool = false;
                            for mm2 in hfj {
                                hfj_vec.push(mm2.to_string());
                            }
                            while i91 < hfj_vec.len() {
                                if ya == false {
                                    if hfj_vec[i91] == "#+#" {
                                        index_macro = i91;
                                        ya = true;
                                    }
                                }
                                i91 += 1;
                            }
                            if !utils::es_numero(hfj_vec[index_macro + 1].trim().to_string()) {
                                return Some(ErrorLeng::new(
                                    "esta macro es de sumar.no se permite texto solo numeros. :("
                                        .to_string(),
                                    programa,
                                    false,
                                ));
                            } else {
                                let num1: isize = hfj_vec[index_macro + 1].parse().unwrap();
                                if !utils::es_numero(hfj_vec[index_macro + 2].trim().to_string()) {
                                    return Some(ErrorLeng::new(
                                    "esta macro es de sumar.no se permite texto solo numeros. :("
                                        .to_string(),
                                    programa,
                                    false,
                                ));
                                } else {
                                    let num2: isize = hfj_vec[index_macro + 2].parse().unwrap();

                                    linea_String = linea_String
                                        .replace(
                                            &format!("#+# {} {}", num1, num2),
                                            &format!("{}", num1 + num2),
                                        )
                                        .to_string();
                                    //println!("'{}','{}'", num1, num2);
                                }
                            }
                            //linea = &linea_String;
                            jb = linea_String.contains("#+#");
                        }
                    }
                    if linea_String.contains("#/#") {
                        let mut jb: bool = linea_String.contains("#/#");
                        //println!("'{}'", "j");
                        while jb == true {
                            //println!("{}", linea_String);
                            //let index_macro_init = linea_String.find("#+#").unwrap();
                            let mut hfj = linea_String.split(" ");
                            let mut hfj_vec: Vec<String> = Vec::new();
                            let mut index_macro: usize = 0;
                            let mut i91: usize = 0;
                            let mut ya: bool = false;
                            for mm2 in hfj {
                                hfj_vec.push(mm2.to_string());
                            }
                            while i91 < hfj_vec.len() {
                                if ya == false {
                                    if hfj_vec[i91] == "#/#" {
                                        index_macro = i91;
                                        ya = true;
                                    }
                                }
                                i91 += 1;
                            }
                            if !utils::es_numero(hfj_vec[index_macro + 1].trim().to_string()) {
                                return Some(ErrorLeng::new(
                                    "esta macro es de dividir.no se permite texto solo numeros. :("
                                        .to_string(),
                                    programa,
                                    false,
                                ));
                            } else {
                                let num1: isize = hfj_vec[index_macro + 1].parse().unwrap();
                                if !utils::es_numero(hfj_vec[index_macro + 2].trim().to_string()) {
                                    return Some(ErrorLeng::new(
                                    "esta macro es de dividir.no se permite texto solo numeros. :("
                                        .to_string(),
                                    programa,
                                    false,
                                ));
                                } else {
                                    let num2: isize = hfj_vec[index_macro + 2].parse().unwrap();

                                    linea_String = linea_String
                                        .replace(
                                            &format!("#/# {} {}", num1, num2),
                                            &format!("{}", num1 / num2),
                                        )
                                        .to_string();
                                    //println!("'{}','{}'", num1, num2);
                                }
                            }
                            //linea = &linea_String;
                            jb = linea_String.contains("#/#");
                        }
                    }

                    if linea_String.contains("#*#") {
                        let mut jb: bool = linea_String.contains("#*#");
                        //println!("'{}'", "j");
                        while jb == true {
                            //println!("{}", linea_String);
                            //let index_macro_init = linea_String.find("#+#").unwrap();
                            let mut hfj = linea_String.split(" ");
                            let mut hfj_vec: Vec<String> = Vec::new();
                            let mut index_macro: usize = 0;
                            let mut i91: usize = 0;
                            let mut ya: bool = false;
                            for mm2 in hfj {
                                hfj_vec.push(mm2.to_string());
                            }
                            while i91 < hfj_vec.len() {
                                if ya == false {
                                    if hfj_vec[i91] == "#*#" {
                                        index_macro = i91;
                                        ya = true;
                                    }
                                }
                                i91 += 1;
                            }
                            if !utils::es_numero(hfj_vec[index_macro + 1].trim().to_string()) {
                                return Some(ErrorLeng::new(
                                    "esta macro es de multiplicar.no se permite texto solo numeros. :("
                                        .to_string(),
                                    programa,
                                    false,
                                ));
                            } else {
                                let num1: isize = hfj_vec[index_macro + 1].parse().unwrap();
                                if !utils::es_numero(hfj_vec[index_macro + 2].trim().to_string()) {
                                    return Some(ErrorLeng::new(
                                    "esta macro es de multiplicar.no se permite texto solo numeros. :("
                                        .to_string(),
                                    programa,
                                    false,
                                ));
                                } else {
                                    let num2: isize = hfj_vec[index_macro + 2].parse().unwrap();

                                    linea_String = linea_String
                                        .replace(
                                            &format!("#*# {} {}", num1, num2),
                                            &format!("{}", num1 * num2),
                                        )
                                        .to_string();
                                    //println!("'{}','{}'", num1, num2);
                                }
                            }
                            //linea = &linea_String;
                            jb = linea_String.contains("#*#");
                        }
                    }

                    //resta

                    if linea_String.contains("#-#") {
                        let mut jb: bool = linea_String.contains("#-#");
                        //println!("'{}'", "j");
                        while jb == true {
                            //println!("{}", linea_String);
                            //let index_macro_init = linea_String.find("#+#").unwrap();
                            let mut hfj = linea_String.split(" ");
                            let mut hfj_vec: Vec<String> = Vec::new();
                            let mut index_macro: usize = 0;
                            let mut i91: usize = 0;
                            let mut ya: bool = false;
                            for mm2 in hfj {
                                hfj_vec.push(mm2.to_string());
                            }
                            while i91 < hfj_vec.len() {
                                if ya == false {
                                    if hfj_vec[i91] == "#-#" {
                                        index_macro = i91;
                                        ya = true;
                                    }
                                }
                                i91 += 1;
                            }
                            if !utils::es_numero(hfj_vec[index_macro + 1].trim().to_string()) {
                                return Some(ErrorLeng::new(
                                    "esta macro es de restar.no se permite texto solo numeros. :("
                                        .to_string(),
                                    programa,
                                    false,
                                ));
                            } else {
                                let num1: isize = hfj_vec[index_macro + 1].parse().unwrap();
                                if !utils::es_numero(hfj_vec[index_macro + 2].trim().to_string()) {
                                    return Some(ErrorLeng::new(
                                    "esta macro es de restar.no se permite texto solo numeros. :("
                                        .to_string(),
                                    programa,
                                    false,
                                ));
                                } else {
                                    let num2: isize = hfj_vec[index_macro + 2].parse().unwrap();

                                    linea_String = linea_String
                                        .replace(
                                            &format!("#-# {} {}", num1, num2),
                                            &format!("{}", num1 - num2),
                                        )
                                        .to_string();
                                    //println!("'{}','{}'", num1, num2);
                                }
                            }
                            //linea = &linea_String;
                            jb = linea_String.contains("#-#");
                        }
                    }
                }

                linea = &linea_String;
                let mut es_variable: bool = false;
                //let ini40:String = linea.sta
                //no funciones
                //if linea.to_string().ends_with(")") ==
                for inicio in programa.get_variables_all_nombre() {
                    if linea.starts_with(&inicio) {
                        es_variable = true;
                    }
                }

                let mut es_variable_vector: bool = false;
                //let ini40:String = linea.sta
                //no funciones
                //if linea.to_string().ends_with(")") ==
                let mut nom_vector: String = "".to_string();
                for inicio in programa.get_vector_variable_all_nombre() {
                    if linea.starts_with(&inicio) {
                        es_variable_vector = true;
                        nom_vector = inicio;
                    }
                }

                let mut push_vector_variable: bool = false;
                if es_variable_vector {
                    if linea.starts_with(&format!("{}.agregar(", nom_vector.clone())) {
                        push_vector_variable = true;
                    }
                }
                if linea.starts_with("var:") {
                    let mut hfj = linea.split("=");
                    let mut hfj_vec: Vec<String> = Vec::new();
                    for mm2 in hfj {
                        hfj_vec.push(mm2.to_string());
                    }

                    if utils::verificar_len(hfj_vec.clone(), 2) == false {
                        return Some(ErrorLeng::new(
                            "estructura incorrecta pruebe esta: 'var: NOMBRE = VALOR'".to_string(),
                            programa3.clone(),
                            false,
                        ));
                    } else {
                        let nombre_var_3: String =
                            hfj_vec[0].trim().to_string()[5..hfj_vec[0].len() - 1].to_string();
                        if programa.get_variable(nombre_var_3.clone()).is_some() {
                            return Some(ErrorLeng::new(
                                "la variable ya existe :(".to_string(),
                                programa3.clone(),
                                false,
                            ));
                        } else {
                            //println!("'{}'", nombre_var_3);
                            programa.add_variable(
                                //ojooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo posible bug en 4..hfj_vec[0].len() - 1
                                nombre_var_3,
                                hfj_vec[1].trim().to_string(),
                            );
                        }
                    }
                    //if let Some(arg1) = hfj1.nth(0) {}
                } else if push_vector_variable {
                    if !linea.ends_with(")") {
                        return Some(ErrorLeng::new(
                            "la funcion no termino en: ')' ".to_string(),
                            programa5.clone(),
                            false,
                        ));
                    }
                    let mut nuevo_elemento: String =
                        linea[nom_vector.len() + 9..linea.len() - 1].to_string();
                    nuevo_elemento = nuevo_elemento.trim().to_string();
                    programa.add_to_vector_element(nom_vector, nuevo_elemento);
                } else if es_variable {
                    let mut hfj = linea.split("=");
                    let mut hfj_vec: Vec<String> = Vec::new();
                    for mm2 in hfj {
                        hfj_vec.push(mm2.to_string());
                    }
                    if utils::verificar_len(hfj_vec.clone(), 2) == false {
                        return Some(ErrorLeng::new(
                            "estructura incorrecta pruebe esta: 'NOMBRE = VALOR'".to_string(),
                            programa3.clone(),
                            false,
                        ));
                    } else {
                        let nombre_var_3: String = hfj_vec[0].trim().to_string();
                        programa.set_variable(programa.clone(), nombre_var_3, hfj_vec[1].clone());
                        //println!("'{}'", nombre_var_3);
                    }
                }
                //lista: numeros-1-10
                else if linea.starts_with("lista:") {
                    let mut nombre_vector: String = linea[6..linea.len()].to_string();
                    nombre_vector = nombre_vector.trim().to_string();
                    if programa
                        .get_vector_variable(nombre_vector.clone())
                        .is_some()
                    {
                        return Some(ErrorLeng::new(
                            "ya existe la lista".to_string(),
                            programa.clone(),
                            false,
                        ));
                    } else {
                        programa.create_add_vector_element(nombre_vector.clone());
                    }
                } else if linea.starts_with("bucle") {
                    let mut hfj = linea.split(":");
                    let mut hfj_vec: Vec<String> = Vec::new();
                    for mm2 in hfj {
                        hfj_vec.push(mm2.trim().to_string());
                    }
                    // println!("{}", "m");
                    let numero3 = hfj_vec[0][6..hfj_vec[0].len()].parse::<usize>();

                    //println!("{}", "m");
                    if numero3.is_err() {
                        //println!("{}", "m");
                        return Some(ErrorLeng::new(
                            "el valor pasado en el bucle no es un numero".to_string(),
                            programa,
                            false,
                        ));
                    } else {
                        //println!("{}", "m");
                        let numero: usize = numero3.unwrap();
                        let var: String = hfj_vec[1].clone();
                        let mut i = 0;

                        let mut mkkk3 = programa.set_variable2(var.clone(), "0".to_string());

                        if mkkk3.err2() {
                            return Some(mkkk3);
                        }
                        //println!("{}", "m");
                        if programa.get_variable(var.clone()).is_none() {
                            return Some(ErrorLeng::new(
                                "no se encontro la variable".to_string(),
                                programa.clone(),
                                false,
                            ));
                        }

                        while i < numero {
                            //println!("{}", i);
                            //programa.set_variable2(var.clone(), format!("{}", i));
                            let err: Option<ErrorLeng> =
                                Programa::init_program(programa.clone(), hfj_vec[2].clone());
                            if err.is_some() {
                                return Some(err.unwrap());
                            } //************************************************* */
                            i += 1;
                            programa.set_variable2(var.clone(), format!("{}", i));
                        }
                    }
                } else {
                    //funciones
                    if linea.to_string().ends_with(")") == false {
                        return Some(ErrorLeng::new(
                            format!("no se termino en ) la funcion:[{}]", linea),
                            programa,
                            false,
                        ));
                    }
                    if programa6.clone().detener == false {
                        let mut c2 = linea.split("(");

                        if let Some(arg1) = c2.nth(0) {
                            let mut k24 = arg1.split(".");
                            let mut k25 = arg1.split(".");
                            if let Some(prefix) = k24.nth(0) {
                                //mirar si aqui esta el prefix falta programarlo
                                //println!("dkd");

                                if let Some(comando) = k25.nth(1) {
                                    //aquiiiiiii esta el errrrrrrrrroororororororororororororrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr
                                    //println!("dkd");
                                    let mut ch4_comando: String = comando.to_string();
                                    if ch4_comando.ends_with(")") {
                                        ch4_comando.pop();
                                    }
                                    let mut params1: String = linea
                                        [ch4_comando.len() + 2 + prefix.len()..linea.len() - 1]
                                        .to_string();
                                    params1 = params1.trim().to_string();
                                    let params2 = params1.split("ª");
                                    let mut params: Vec<String> = Vec::new();
                                    for pa in params2 {
                                        params.push(pa.trim().to_string());
                                    }
                                    //println!("'{}'", prefix.to_string().trim().to_string());
                                    let mut eje = programa
                                        .get_for_prefix(prefix.to_string().trim().to_string())
                                        .unwrap()
                                        .funcion(ch4_comando)
                                        .unwrap()(
                                        params, programa.clone()
                                    );
                                    if eje.is_err() {
                                        //error_mas_reciente = eje;
                                        return Some(ErrorLeng::new(
                                            eje.err().unwrap().message(),
                                            programa,
                                            false,
                                        ));
                                    } else {
                                        programa = eje.ok().unwrap();
                                    }
                                }
                            }
                        }
                    }
                }
            }

            //c2.next()
        }
        return None;
    }
}
