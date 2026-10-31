use crate::alumnos_registrados::IdAlumno;
use std::time::SystemTime;

pub struct Clase {
    alumno_id: IdAlumno,
    aviso_cancelacion: Option<SystemTime>,
}

impl Clase {
    pub(crate) fn new(alumno_id: IdAlumno) -> Self {
        Clase {
            alumno_id,
            aviso_cancelacion: None,
        }
    }
}
