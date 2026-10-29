use crate::estado_clase::EstadoClase;

pub struct Clase {
    alumno_id: u32,
    estado: EstadoClase,
}

impl Clase {
    pub(crate) fn new(alumno_id: u32) -> Self {
        Clase {
            alumno_id,
            estado: EstadoClase::new_activa(),
        }
    }
}
