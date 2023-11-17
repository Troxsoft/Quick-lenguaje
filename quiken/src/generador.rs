use std::{fs, io};

pub fn generar_proyecto(name: String) -> io::Result<()> {
    fs::create_dir_all(format!("{}/codigo", name))
}
pub fn generar_main(name: String) -> io::Result<()> {
    fs::write(
        format!("{}/codigo/principal.qk", name),
        "escribir(hola mundo)",
    )
}
pub fn generar_proyecto2(name: String) -> io::Result<()> {
    fs::create_dir(format!("{}/dependencias", name))
}
pub fn generar_depe(name: String) -> io::Result<()> {
    fs::write(
        format!("{}/dependencias/depe.txt", name),
        "
std_lib
    ",
    )
}
