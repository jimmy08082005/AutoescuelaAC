pub struct Alumno {
    dni: String,
    nombre: String,
    direccion: Direccion,
}

impl Alumno {
    pub fn new(dni: String, nombre: String, direccion: Direccion) -> Result<Self, ErrorAutoescuela> {
        let valido = dni.len() == 9
        && dni[..8].chars().all(|c| c.is_ascii_digit())
        && dni.chars().nth(8).map_or(false, |c| c.is_ascii_alphabetic());
        
        
        if !valido {
        
            return Err(ErrorAutoescuela::DniInvalido {
            
            error: format!("'{}' DNI incorrecto, un DNI se compone por 8 cifras y una letra", dni),
            
            });
        
        }

        Ok(Alumno { dni, nombre, direccion })
    }

    pub fn dni(&self) -> &str {
        &self.dni
    }

    pub fn nombre(&self) -> &str {
        &self.nombre
    }

    pub fn direccion(&self) -> &Direccion {
        &self.direccion
    }
}