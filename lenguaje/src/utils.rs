pub fn verificar_len(params: Vec<String>, len: usize) -> bool {
    let mut h: bool = false;
    if params.len() == len {
        h = true;
    }
    return h;
}

pub fn es_numero(txt: String) -> bool {
    if txt.parse::<isize>().is_err() {
        return false;
    } else {
        return true;
    }
}

pub fn es_txt(txt: String) -> bool {
    return !es_numero(txt);
}
