use std::collections::HashMap;

pub struct AlumnosRegistrados {
    alumnos: HashMap<String, Alumno>,
}

impl AlumnosRegistrados {
    pub fn new() -> Self {
        AlumnosRegistrados { alumnos: HashMap::new() }
    }

    pub fn registrar(&mut self, alumno: Alumno) -> Result<(), ErrorAutoescuela> {
        let dni = alumno.dni().to_string();

        if self.alumnos.contains_key(&dni) {
            return Err(ErrorAutoescuela::AlumnoDuplicado {
                error: format!("Ya existe un alumno registrado con el DNI {}", dni),
            });
        }

        self.alumnos.insert(dni, alumno);
        Ok(())
    }

    pub fn existe(&self, dni: &str) -> bool {
        self.alumnos.contains_key(dni)
    }
}