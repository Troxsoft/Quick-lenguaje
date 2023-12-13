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
    fun.add_funcion(eliminar_std_var, "eliminar_var".to_string());
    fun.add_funcion(eliminar_std_vector, "eliminar_lista".to_string());
    fun.add_funcion(eliminar_std_vector_index, "eliminar_indice".to_string());
    //fun.add_funcion(escribir_color_fondo_std, "escribir_colorf".to_string());
    fun.add_funcion(get_vector_indice, "obt_indice".to_string());
    fun.add_funcion(escribir_std_vector, "escribir_lista".to_string());

    fun.add_funcion(mayor_numero_std, "es_mayor".to_string());
    fun.add_funcion(menor_numero_std, "es_menor".to_string());
    fun.add_funcion(igual_txt_numero_std, "es_igual".to_string());
    fun
}


fn menor_numero_std(params: Vec<String>, mut programa: Programa) -> Result<Programa, ErrorLeng> {
    if utils::verificar_len(params.clone(), 2) == false {
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
    if utils::es_numero(params[0].clone())==false{
        return  Err(ErrorLeng::new("el supuesto numero '1' no es un numero :( PRUEBA ESTA SINTAXIS 'std.es_mayor( @[n1] ª @[n2] )' ".to_string(), programa.clone(), false));
    }
    if utils::es_numero(params[1].clone())==false{
        return  Err(ErrorLeng::new("el supuesto numero '2' no es un numero :( PRUEBA ESTA SINTAXIS 'std.es_mayor( @[n1] ª @[n2] )' ".to_string(), programa.clone(), false));
    }
    let mut pro = programa.clone();
    if params[0].parse::<isize>().ok().unwrap() < params[1].parse::<isize>().ok().unwrap(){
        pro.set_variable(pro.clone(),"ret".to_string(), "verdadero".to_string());
    }else{
        pro.set_variable(pro.clone(),"ret".to_string(), "falso".to_string());
    }
    return Ok(pro);
}


               









fn igual_txt_numero_std(params: Vec<String>, mut programa: Programa) -> Result<Programa, ErrorLeng> {
    if utils::verificar_len(params.clone(), 2) == false {
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
    
    let mut pro = programa.clone();
    if params[0] == params[1]{
        pro.set_variable(pro.clone(),"ret".to_string(), "verdadero".to_string());
    }else{
        pro.set_variable(pro.clone(),"ret".to_string(), "falso".to_string());
    }
    return Ok(pro);
}
fn mayor_numero_std(params: Vec<String>, mut programa: Programa) -> Result<Programa, ErrorLeng> {
    if utils::verificar_len(params.clone(), 2) == false {
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
    if utils::es_numero(params[0].clone())==false{
        return  Err(ErrorLeng::new("el supuesto numero '1' no es un numero :( PRUEBA ESTA SINTAXIS 'std.es_mayor( @[n1] ª @[n2] )' ".to_string(), programa.clone(), false));
    }
    if utils::es_numero(params[1].clone())==false{
        return  Err(ErrorLeng::new("el supuesto numero '2' no es un numero :( PRUEBA ESTA SINTAXIS 'std.es_mayor( @[n1] ª @[n2] )' ".to_string(), programa.clone(), false));
    }
    let mut pro = programa.clone();
    if params[0].parse::<isize>().ok().unwrap() > params[1].parse::<isize>().ok().unwrap(){
        pro.set_variable(pro.clone(),"ret".to_string(), "verdadero".to_string());
    }else{
        pro.set_variable(pro.clone(),"ret".to_string(), "falso".to_string());
    }
    return Ok(pro);
}

fn eliminar_std_vector(params: Vec<String>, mut programa: Programa) -> Result<Programa, ErrorLeng> {
    if utils::verificar_len(params.clone(), 1) == false {
        return Err(ErrorLeng::new(
            "no se recibio la cantidad correcta de parametros".to_string(),
            programa.clone(),
            false,
        ));
    }
    if !programa.delete_vector(params[0].clone()) {
        return Err(ErrorLeng::new(
            "hubo un error al eliminar la variable ".to_string(),
            programa,
            false,
        ));
    }
    return Ok(programa);
}

fn eliminar_std_vector_index(
    params: Vec<String>,
    mut programa: Programa,
) -> Result<Programa, ErrorLeng> {
    if utils::verificar_len(params.clone(), 2) == false {
        return Err(ErrorLeng::new(
            "no se recibio la cantidad correcta de parametros".to_string(),
            programa.clone(),
            false,
        ));
    }
    if utils::es_numero(params[1].clone()) == false {
        return Err(ErrorLeng::new(
            "el indice a eliminar debe ser un numero, pero se recibio un texto :(".to_string(),
            programa.clone(),
            false,
        ));
    }
    //print!("'{}'", params[1].clone().parse::<usize>().unwrap());
    if !programa.delete_vector_index(params[0].clone(), params[1].clone().parse().unwrap()) {
        return Err(ErrorLeng::new(
            "hubo un error al eliminar la variable ".to_string(),
            programa,
            false,
        ));
    }
    return Ok(programa);
}

fn eliminar_std_var(params: Vec<String>, mut programa: Programa) -> Result<Programa, ErrorLeng> {
    if utils::verificar_len(params.clone(), 1) == false {
        return Err(ErrorLeng::new(
            "no se recibio la cantidad correcta de parametros".to_string(),
            programa.clone(),
            false,
        ));
    }
    if !programa.delete_var(params[0].clone()) {
        return Err(ErrorLeng::new(
            "hubo un error al eliminar la variable ".to_string(),
            programa,
            false,
        ));
    }
    return Ok(programa);
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

fn get_vector_indice(params: Vec<String>, mut programa: Programa) -> Result<Programa, ErrorLeng> {
    let mut params2: Vec<String> = params.clone();
    if utils::verificar_len(params, 2) == false {
        return Err(ErrorLeng::new(
            "la cantidad de parametros no es correcta".to_string(),
            programa.clone(),
            false,
        ));
    }
    if utils::es_numero(params2[1].clone()) == false {
        return Err(ErrorLeng::new(
            "el indice debe ser un numero".to_string(),
            programa.clone(),
            false,
        ));
    }
    let mut pro3: Programa = programa.clone();
    pro3.set_variable(
        pro3.clone(),
        "ret".to_string(),
        pro3.clone()
            .get_vector_variable(params2[0].clone())
            .unwrap()[params2[1].clone().parse::<usize>().unwrap()]
        .clone(),
    );
    return Ok(pro3);
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
fn escribir_std_vector(params: Vec<String>, mut programa: Programa) -> Result<Programa, ErrorLeng> {
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
    } else {
        if programa.get_vector_variable(params[0].clone()).is_none() {
            return Err(ErrorLeng::new(
                "no existe la lista".to_string(),
                programa.clone(),
                false,
            ));
        }
        println!(
            "{:?}",
            programa.get_vector_variable(params[0].clone()).unwrap()
        );
        return Ok(programa);
    }
}
