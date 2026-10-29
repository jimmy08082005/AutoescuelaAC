use std::time::SystemTime;

pub enum EstadoClase {
    Activa,
    Cancelada { instante_aviso: SystemTime },
}

impl EstadoClase {
    pub(crate) fn new_activa() -> Self {
        EstadoClase::Activa
    }
}
