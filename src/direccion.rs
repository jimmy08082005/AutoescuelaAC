pub struct Direccion {
    calle: String,
    numero: String,
    piso: Option<String>,
}

impl Direccion {
    pub fn new(calle: String, numero: String, piso: Option<String>) -> Result<Self, ErrorAutoescuela> {
        if calle.trim().is_empty() || numero.trim().is_empty() {
            return Err(ErrorAutoescuela::DireccionInvalida("La calle y el número no pueden estar vacíos".to_string()));
        }
        Ok(Direccion { calle, numero, piso })
    }

    pub fn calle(&self) -> &str {
        &self.calle
    }

    pub fn numero(&self) -> &str {
        &self.numero
    }

    pub fn piso(&self) -> Option<&String> {
        self.piso.as_ref()
    }
}