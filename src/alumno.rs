use crate::direccion::Direccion;
use crate::errores_dominio::ErrorAutoescuela;

pub struct Alumno {
    nombre: String,
    direccion: Direccion,
}

impl Alumno {
    pub(crate) fn new(nombre: String, direccion: Direccion) -> Result<Self, ErrorAutoescuela> {
        Ok(Alumno { nombre, direccion })
    }
}
