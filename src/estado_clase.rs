use std::time::SystemTime;

pub enum EstadoClase {
    Activa,
    Cancelada { instante_aviso: SystemTime },
}

impl EstadoClase {
    pub(crate) fn new_activa() -> Self {
        EstadoClase::Activa
    }

    pub(crate) fn new_cancelada(instante_aviso: SystemTime) -> Self {
        EstadoClase::Cancelada { instante_aviso }
    }
}
