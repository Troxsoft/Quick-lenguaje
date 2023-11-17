use colored::Colorize;

use super::qlenguaje::Programa;

pub struct ErrorLeng {
    message: String,
    programa: Programa,
    ignore: bool,
}

impl ErrorLeng {
    pub fn new(message: String, programa: Programa, ignore: bool) -> Self {
        ErrorLeng {
            message: message,
            programa: programa,
            ignore: ignore,
        }
    }
    pub fn message(&self) -> String {
        return self.message.clone();
    }
    pub fn new_ignore(programa: Programa) -> Self {
        ErrorLeng {
            message: "".to_string(),
            programa: programa,
            ignore: true,
        }
    }
    pub fn err2(&mut self) -> bool {
        if self.ignore == false {
            self.programa.detener = true;
            self.programa.salir = true;
            return true;
        }
        return false;
    }
    pub fn err(&mut self) -> bool {
        if self.ignore == false {
            println!(
                "
           {}
                @mensaje
           {}
        ",
                "Error en ejecucion
                --------        SE DETUVO LA EJECUCION"
                    .red(),
                self.message.blue()
            );
            self.programa.detener = true;
            self.programa.salir = true;
            return true;
        }
        return false;
    }
}
