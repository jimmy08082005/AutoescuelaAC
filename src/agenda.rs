use crate::alumnos_registrados::AlumnosRegistrados;
use crate::clase::Clase;
use crate::errores_dominio::ErrorAutoescuela;
use crate::ubicacion_coche::UbicacionCoche;
use std::collections::BTreeMap;
use std::ops::Bound::Excluded;
use std::time::{Duration, SystemTime};

const DURACION_CLASE_SEGUNDOS: u64 = 45 * 60;

pub struct Agenda {
    clases: BTreeMap<SystemTime, Clase>,
    ubicacion_coche: Option<UbicacionCoche>,
}

impl Agenda {
    pub fn new() -> Self {
        Agenda {
            clases: BTreeMap::new(),
            ubicacion_coche: None,
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

        let margen = Duration::from_secs(DURACION_CLASE_SEGUNDOS);
        let desde = hora_inicio
            .checked_sub(margen)
            .unwrap_or(SystemTime::UNIX_EPOCH);
        let hasta = hora_inicio + margen;

        if self
            .clases
            .range((Excluded(desde), Excluded(hora_inicio)))
            .next()
            .is_some()
            || self.clases.range(hora_inicio..hasta).next().is_some()
        {
            return Err(ErrorAutoescuela::ClaseSolapada);
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
        self.ubicacion_coche = Some(ubicacion);
        Ok(())
    }
}
