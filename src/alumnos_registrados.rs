use crate::alumno::Alumno;
use crate::direccion::Direccion;
use crate::errores_dominio::ErrorAutoescuela;
use std::collections::HashMap;

pub struct AlumnosRegistrados {
    alumnos: HashMap<u32, Alumno>,
    siguiente_id: u32,
}

impl AlumnosRegistrados {
    pub fn new() -> Self {
        AlumnosRegistrados {
            alumnos: HashMap::new(),
            siguiente_id: 1,
        }
    }

    pub fn registrar(
        &mut self,
        nombre: String,
        calle: String,
        numero: String,
        piso: Option<String>,
    ) -> Result<u32, ErrorAutoescuela> {
        let id = self.siguiente_id;
        let direccion = Direccion::new(calle, numero, piso)?;
        let alumno = Alumno::new(nombre, direccion);
        self.alumnos.insert(id, alumno);
        self.siguiente_id += 1;
        Ok(id)
    }
}
