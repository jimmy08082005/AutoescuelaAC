use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorAutoescuela {
    DireccionInvalida,
    CancelacionFueraPlazo,
    UbicacionInvalida,
    AlumnoDuplicado,
    DniInvalido,
    ClaseSolapada,
}

impl fmt::Display for ErrorAutoescuela {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mensaje = match self {
            Self::DireccionInvalida => {
                "La dirección proporcionada no es válida (calle y número no pueden estar vacíos)"
            }
            Self::CancelacionFueraPlazo => {
                "La cancelación se realiza fuera del plazo permitido (antes de la hora de inicio)"
            }
            Self::UbicacionInvalida => {
                "La ubicación especificada no es válida (coordenadas fuera de rango o indefinidas)"
            }
            Self::AlumnoDuplicado => "El alumno ya se encuentra registrado (DNI duplicado)",
            Self::DniInvalido => {
                "El DNI proporcionado no tiene un formato válido (8 dígitos seguidos de una letra)"
            }
            Self::ClaseSolapada => {
                "La clase se solapa con otro horario existente (coincidencia de fechas y horas o no han pasado 45 minutos entre clases)"
            }
        };
        write!(f, "{}", mensaje)
    }
}

impl std::error::Error for ErrorAutoescuela {}
