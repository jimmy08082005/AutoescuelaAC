use crate::direccion::Direccion;

pub struct Alumno {
    nombre: String,
    direccion: Direccion,
}

impl Alumno {
    pub(crate) fn new(nombre: String, direccion: Direccion) -> Self {
        Alumno { nombre, direccion }
    }
}
