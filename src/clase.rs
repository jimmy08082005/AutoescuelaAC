use crate::alumnos_registrados::IdAlumno;
use crate::estado_clase::EstadoClase;

pub struct Clase {
    alumno_id: IdAlumno,
    estado: EstadoClase,
}

impl Clase {
    pub(crate) fn new(alumno_id: IdAlumno) -> Self {
        Clase {
            alumno_id,
            estado: EstadoClase::new_activa(),
        }
    }
}
