use std::time::SystemTime;
use crate::errores_dominio::ErrorAutoescuela;

pub struct Ubicacion {
    latitud: f64,
    longitud: f64,
    instante: SystemTime,
}

impl Ubicacion {
    pub fn new(latitud: f64, longitud: f64, instante: SystemTime) -> Result<Self, ErrorAutoescuela> {
        if latitud.is_nan() || longitud.is_nan() {
            return Err(ErrorAutoescuela::UbicacionInvalida);
        }

        if latitud < -90.0 || latitud > 90.0 {
            return Err(ErrorAutoescuela::UbicacionInvalida);
        }

        if longitud < -180.0 || longitud > 180.0 {
            return Err(ErrorAutoescuela::UbicacionInvalida);
        }

        Ok(Ubicacion {
            latitud,
            longitud,
            instante,
        })
    }
}
