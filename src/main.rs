use std::{env, fs, time::Instant};
pub mod comandos;
use lenguaje::{
    funciones::fstructure::Funciones,
    leng::{error_leng::ErrorLeng, qlenguaje::Programa},
    std_quick::get_std,
};

const VERSION: &str = "0.0.1";
fn main() {
    if let Some(arg1) = env::args().nth(1) {
        if arg1 == "crear" {
            if let Some(arg2) = env::args().nth(2) {
                let jo = arg2.clone();
                let joe = arg2.clone();
                let jo3 = arg2.clone();
                quiken::generador::generar_proyecto(arg2).ok();
                quiken::generador::generar_main(jo).ok();
                quiken::generador::generar_proyecto2(joe).ok();
                quiken::generador::generar_depe(jo3).ok();
            } else {
                println!(
                    "
                    COMANDO EQUIVOCADO: [{}]
            Usa el comando:'ayuda' para obtener ayuda
                            @te falta el nombre del proyecto
            ",
                    arg1
                )
            }
        } else if arg1 == "ayuda" {
            crate::comandos::ayuda::comando_help(VERSION, quiken::VERSION);
        } else if arg1 == "ejecuta" {
            //println!("s");
            let mut programa: Programa = Programa::new();

            programa.add_funcion(get_std());
            let mut programa2: Programa = programa.clone();
            if let Some(arg2) = env::args().nth(2) {
                let h29 = arg2.clone();

                let codigo: String = fs::read_to_string(h29).unwrap();
                let codigo2: String = codigo.clone();

                Programa::init_program(programa, codigo)
                    .unwrap_or_else(|| -> ErrorLeng {
                        return ErrorLeng::new_ignore(programa2);
                    })
                    .err();
            } else {
                println!(
                    "
                        COMANDO EQUIVOCADO: [{}] 
                Usa el comando:'ayuda' para obtener ayuda
                                @la ruta es incorrecta
                ",
                    arg1
                );
            }
        } else if arg1 == "ejecuta-tiempo" {
            //println!("s");
            let now = Instant::now();
            let mut programa: Programa = Programa::new();

            programa.add_funcion(get_std());
            let mut programa2: Programa = programa.clone();
            if let Some(arg2) = env::args().nth(2) {
                let h29 = arg2.clone();

                let codigo: String = fs::read_to_string(h29).unwrap();
                let codigo2: String = codigo.clone();

                Programa::init_program(programa, codigo)
                    .unwrap_or_else(|| -> ErrorLeng {
                        return ErrorLeng::new_ignore(programa2);
                    })
                    .err();
            } else {
                println!(
                    "
                        COMANDO EQUIVOCADO: [{}] 
                Usa el comando:'ayuda' para obtener ayuda
                                @la ruta es incorrecta
                ",
                    arg1
                );
            }
            println!(
                "
            tiempo transcurrido: {} segundos
            tiempo transcurrido: {} en milisegundos
            tiempo transcurrido: {} en nanosegundos
            
            ",
                now.elapsed().as_secs(),
                now.elapsed().as_millis(),
                now.elapsed().as_nanos()
            );
        }
    } else {
        println!(
            "
                    Hola Programador :)
        -- La version de Quick es: [{}]             [Utiliza el comando ayuda si no sabes como funciona]
        -- y la de Quiken(administrador de proyector de quick) es: [{}]

        ",
            VERSION,
            quiken::VERSION
        );
    }
    /*  let mut std_funciones: Funciones = Funciones::new("std".to_string());
    std_funciones = convert_to_std(std_funciones); */

    //std_funciones.funcion("escribir".to_string()).unwrap()(vec!["m".to_string()]);
}
