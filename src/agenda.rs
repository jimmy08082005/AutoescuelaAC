use std::collections::HashMap;
use std::time::SystemTime;
use crate::clase::Clase;
use crate::errores_dominio::ErrorAutoescuela;
use crate::alumnos_registrados::AlumnosRegistrados;

pub struct Agenda {
    clases: HashMap<SystemTime, Clase>,
}

impl Agenda {
    pub fn new() -> Self {
        Agenda {
            clases: HashMap::new(),
        }
    }

    pub fn agregar_clase(&mut self, clase: Clase, alumnos: &AlumnosRegistrados) -> Result<(), ErrorAutoescuela> {
        if !alumnos.existe(&clase.dni_alumno()) {
            return Err(ErrorAutoescuela::AlumnoNoRegistrado);
        }

        if self.clases.contains_key(&clase.hora_inicio()) {
            return Err(ErrorAutoescuela::ClaseSolapada);
        }

        self.clases.insert(*clase.hora_inicio(), clase);
        Ok(())
    }
}
