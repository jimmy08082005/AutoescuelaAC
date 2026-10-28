use crate::alumno::Alumno;
use crate::direccion::Direccion;
use crate::errores_dominio::ErrorAutoescuela;
use std::collections::HashMap;

pub struct AlumnosRegistrados {
    alumnos: HashMap<String, Alumno>,
}

impl AlumnosRegistrados {
    pub fn new() -> Self {
        AlumnosRegistrados {
            alumnos: HashMap::new(),
        }
    }

    pub fn registrar(
        &mut self,
        dni: String,
        nombre: String,
        direccion: Direccion,
    ) -> Result<(), ErrorAutoescuela> {
        if self.alumnos.contains_key(&dni) {
            return Err(ErrorAutoescuela::AlumnoDuplicado);
        }

        let alumno = Alumno::new(dni.clone(), nombre, direccion)?;
        self.alumnos.insert(dni, alumno);
        Ok(())
    }

}
