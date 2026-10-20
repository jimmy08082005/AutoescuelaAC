use crate::errores_dominio::ErrorAutoescuela;
use crate::estado_clase::EstadoClase;
use std::time::SystemTime;

pub struct Clase {
    hora_inicio: SystemTime,
    dni_alumno: String,
    estado: EstadoClase,
}

impl Clase {
    pub fn new(hora_inicio: SystemTime, dni_alumno: String) -> Self {
        Clase {
            hora_inicio,
            dni_alumno,
            estado: EstadoClase::new_activa(),
        }
    }

    pub fn cancelar(&mut self, instante_aviso: SystemTime) -> Result<(), ErrorAutoescuela> {
        if instante_aviso >= self.hora_inicio {
            return Err(ErrorAutoescuela::CancelacionFueraPlazo);
        }

        self.estado = EstadoClase::new_cancelada(instante_aviso);
        Ok(())
    }
}
