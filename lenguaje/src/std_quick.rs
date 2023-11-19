use crate::{
    funciones::fstructure::Funciones,
    leng::{error_leng::ErrorLeng, qlenguaje::Programa},
    utils,
};
use colored::control;
use colored::Colorize;

pub fn get_std() -> Funciones {
    let _ = control::set_virtual_terminal(true);
    let mut fun: Funciones = Funciones::new("std".to_string());
    fun.add_funcion(error_std, "error".to_string());
    fun.add_funcion(escribir_std, "escribir".to_string());
    fun.add_funcion(escribir_color_txt_std, "escribir_colort".to_string());
    //fun.add_funcion(escribir_color_fondo_std, "escribir_colorf".to_string());
    fun
}
fn error_std(params: Vec<String>, programa: Programa) -> Result<Programa, ErrorLeng> {
    if utils::verificar_len(params.clone(), 1) == false {
        return Err(ErrorLeng::new(
            "
--------------------------------------------
Error ): 
            FALTAN PARAMETROS
    
-------------------------------------------
    "
            .to_string(),
            programa,
            false,
        ));
    }
    return Err(ErrorLeng::new(format!("{}", params[0]), programa, false));
}
fn escribir_std(params: Vec<String>, programa: Programa) -> Result<Programa, ErrorLeng> {
    if utils::verificar_len(params.clone(), 1) == false {
        return Err(ErrorLeng::new(
            "
--------------------------------------------
Error ): 
            FALTAN PARAMETROS
    
-------------------------------------------
    "
            .to_string(),
            programa.clone(),
            false,
        ));
    }
    println!("{}", params[0]);
    return Ok(programa);
}

fn escribir_color_txt_std(params: Vec<String>, programa: Programa) -> Result<Programa, ErrorLeng> {
    let mut programa2 = programa.clone();
    if utils::verificar_len(params.clone(), 2) == false {
        return Err(ErrorLeng::new(
            "faltan parametros o hay demasiados".to_string(),
            programa.clone(),
            false,
        ));
    }
    if params[1] == "rojo" {
        println!("{}", params[0].red());
    } else if params[1] == "azul" {
        println!("{}", params[0].blue());
    } else if params[1] == "amarillo" {
        println!("{}", params[0].yellow());
    } else if params[1] == "verde" {
        println!("{}", params[0].green());
    } else if params[1] == "morado" {
        println!("{}", params[0].purple());
    } else if params[1] == "blanco" {
        println!("{}", params[0].white());
    } else if params[1] == "negro" {
        println!("{}", params[0].black());
    } else if params[1] == "azul claro" {
        println!("{}", params[0].cyan());
    } else {
        return Err(ErrorLeng::new(
            "color no valido o no implementado en esta funcion".to_string(),
            programa,
            false,
        ));
    }
    return Err(ErrorLeng::new_ignore(programa2));
}
