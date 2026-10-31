pub struct Direccion {
    calle: String,
    numero: String,
}

impl Direccion {
    pub(crate) fn new(calle: String, numero: String) -> Self {
        Direccion { calle, numero }
    }
}
