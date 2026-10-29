use crate::errores_dominio::ErrorAutoescuela;

pub struct Direccion {
    calle: String,
    numero: String,
    piso: Option<String>,
}

impl Direccion {
    pub(crate) fn new(
        calle: String,
        numero: String,
        piso: Option<String>,
    ) -> Result<Self, ErrorAutoescuela> {
        if calle.trim().is_empty() || numero.trim().is_empty() {
            return Err(ErrorAutoescuela::DireccionInvalida);
        }
        Ok(Direccion {
            calle,
            numero,
            piso,
        })
    }
}
