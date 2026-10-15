use crate::direccion::Direccion;
use crate::errores_dominio::ErrorAutoescuela;

pub struct Alumno {
    dni: String,
    nombre: String,
    direccion: Direccion,
}

impl Alumno {
    pub fn new(
        dni: String,
        nombre: String,
        direccion: Direccion,
    ) -> Result<Self, ErrorAutoescuela> {
        let chars: Vec<char> = dni.chars().collect();

        let valido = chars.len() == 9
            && chars[..8].iter().all(|c| c.is_ascii_digit())
            && chars[8].is_ascii_alphabetic();

        if !valido {
            return Err(ErrorAutoescuela::DniInvalido);
        }

        Ok(Alumno {
            dni,
            nombre,
            direccion,
        })
    }

    pub fn dni(&self) -> &str {
        &self.dni
    }
}
