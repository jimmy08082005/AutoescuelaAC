use std::time::SystemTime;

pub struct UbicacionCoche {
    latitud: f64,
    longitud: f64,
    instante: SystemTime,
}

impl UbicacionCoche {
    pub(crate) fn new(latitud: f64, longitud: f64, instante: SystemTime) -> Self {
        UbicacionCoche {
            latitud,
            longitud,
            instante,
        }
    }
}
