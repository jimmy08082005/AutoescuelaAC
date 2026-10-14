#[derive(Debug)]
pub enum ErrorAutoescuela {

    DireccionInvalida { error: String },
    CancelacionFueraPlazo { error: String },
    UbicacionInvalida { error: String },
    AlumnoDuplicado { error: String },
    DniInvalido { error: String },
    ClaseSolapada { error: String },
    AlumnoNoRegistrado { error: String },
    HoraInvalida { error: String },
    FechaInvalida { error: String },
}