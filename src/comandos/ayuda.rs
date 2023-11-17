pub const HELP: &str = "                Ayuda de Quick, Version: @ver@
                Ayuda de Quiken,Version @ver2@
Como crear un proyecto?
    Utiliza: 'Quick.exe crear NOMBRE_PROYECTO'
        Esto creara la siguiente estructura de carpetas
            NOMBRE_PROYECTO
            |--codigo
            |    |--principal.qk
            |--dependencias
                |--depe.txt
";
pub fn format_help(version: &str, version2: &str) -> String {
    let mut f = HELP.replace("@ver@", version);
    f = f.replace("@ver2@", version2);
    let f2 = f;
    f2
}
pub fn comando_help(version: &str, version2: &str) {
    println!("{}", format_help(version, version2));
}
