use crate::alumno::Alumno;
use crate::direccion::Direccion;
use crate::errores_dominio::ErrorAutoescuela;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdAlumno(u32);

pub struct AlumnosRegistrados {
    alumnos: HashMap<IdAlumno, Alumno>,
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
    ) -> Result<IdAlumno, ErrorAutoescuela> {
        let id = IdAlumno(self.siguiente_id);
        let direccion = Direccion::new(calle, numero, piso)?;
        let alumno = Alumno::new(nombre, direccion);
        self.alumnos.insert(id, alumno);
        self.siguiente_id += 1;
        Ok(id)
    }
}
