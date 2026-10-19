use crate::errores_dominio::ErrorAutoescuela;
use std::time::SystemTime;

pub struct UbicacionCoche {
    latitud: f64,
    longitud: f64,
    instante: SystemTime,
}

impl UbicacionCoche {
    pub fn new(
        latitud: f64,
        longitud: f64,
        instante: SystemTime,
    ) -> Result<Self, ErrorAutoescuela> {
        if latitud.is_nan() || longitud.is_nan() {
            return Err(ErrorAutoescuela::UbicacionInvalida);
        }

        if latitud < -90.0 || latitud > 90.0 {
            return Err(ErrorAutoescuela::UbicacionInvalida);
        }

        if longitud < -180.0 || longitud > 180.0 {
            return Err(ErrorAutoescuela::UbicacionInvalida);
        }

        Ok(UbicacionCoche {
            latitud,
            longitud,
            instante,
        })
    }
}
