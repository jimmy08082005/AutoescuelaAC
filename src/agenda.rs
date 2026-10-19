use crate::alumnos_registrados::AlumnosRegistrados;
use crate::clase::Clase;
use crate::errores_dominio::ErrorAutoescuela;
use crate::ubicacion_coche::UbicacionCoche;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

const DURACION_CLASE_SEGUNDOS: u64 = 45 * 60;

pub struct Agenda {
    clases: HashMap<SystemTime, Clase>,
    ubicacion_coche: UbicacionCoche,
}

fn diferencia_absoluta(a: SystemTime, b: SystemTime) -> Duration {
    match a.duration_since(b) {
        Ok(d) => d,
        Err(e) => e.duration(),
    }
}

impl Agenda {
    pub fn new() -> Self {
        Agenda {
            clases: HashMap::new(),
            ubicacion_coche: UbicacionCoche::new(0.0, 0.0, SystemTime::now()).unwrap(),
        }
    }

    pub fn agregar_clase(
        &mut self,
        hora_inicio: SystemTime,
        dni_alumno: String,
        alumnos: &AlumnosRegistrados,
    ) -> Result<(), ErrorAutoescuela> {
        if !alumnos.existe(&dni_alumno) {
            return Err(ErrorAutoescuela::AlumnoNoRegistrado);
        }

        for existente in self.clases.values() {
            let diferencia = diferencia_absoluta(hora_inicio, *existente.hora_inicio());
            if diferencia < Duration::from_secs(DURACION_CLASE_SEGUNDOS) {
                return Err(ErrorAutoescuela::ClaseSolapada);
            }
        }

        let clase = Clase::new(hora_inicio, dni_alumno);
        self.clases.insert(hora_inicio, clase);
        Ok(())
    }

    pub fn modificar_ubicacion_coche(
        &mut self,
        latitud: f64,
        longitud: f64,
        instante: SystemTime,
    ) -> Result<(), ErrorAutoescuela> {
        let ubicacion = UbicacionCoche::new(latitud, longitud, instante)?;
        self.ubicacion_coche = ubicacion;
        Ok(())
    }
}
