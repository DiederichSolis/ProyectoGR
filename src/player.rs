use crate::map::Map;

/// Representa a un jugador en el mapa del juego.
/// El jugador tiene una posición (x, y), una dirección en la que mira (en radianes)
/// y un campo de visión (también en radianes).
pub struct Player {
    pub x: f64,         // Coordenada x de la posición del jugador en el mapa.
    pub y: f64,         // Coordenada y de la posición del jugador en el mapa.
    pub direction: f64, // Dirección en la que está mirando el jugador (en radianes).
    pub fov: f64,       // Campo de visión del jugador (en radianes).
}

impl Player {
    /// Crea un nuevo jugador con una posición y dirección inicial.
    ///
    /// # Argumentos
    ///
    /// * `x` - La posición inicial en el eje x.
    /// * `y` - La posición inicial en el eje y.
    /// * `direction` - La dirección inicial en la que mira el jugador (en radianes).
    ///
    /// # Retorna
    ///
    /// Una nueva instancia de `Player`.
    pub fn new(x: f64, y: f64, direction: f64) -> Self {
        Self {
            x,
            y,
            direction,
            fov: 90.0_f64.to_radians(), // Campo de visión predeterminado de 90 grados.
        }
    }

    /// Mueve al jugador hacia adelante en la dirección que está mirando.
    ///
    /// # Argumentos
    ///
    /// * `distance` - La distancia que el jugador debe moverse.
    /// * `map` - Una referencia al mapa para verificar colisiones con paredes.
    pub fn move_forward(&mut self, distance: f64, map: &Map) {
        let new_x = self.x + self.direction.cos() * distance;
        let new_y = self.y + self.direction.sin() * distance;

        // Verifica si la nueva posición en el eje x no es una pared
        if !map.is_wall(new_x, self.y) {
            self.x = new_x;
        }

        // Verifica si la nueva posición en el eje y no es una pared
        if !map.is_wall(self.x, new_y) {
            self.y = new_y;
        }
    }

    /// Mueve al jugador hacia atrás en la dirección opuesta a la que está mirando.
    ///
    /// # Argumentos
    ///
    /// * `distance` - La distancia que el jugador debe moverse hacia atrás.
    /// * `map` - Una referencia al mapa para verificar colisiones con paredes.
    pub fn move_backward(&mut self, distance: f64, map: &Map) {
        let new_x = self.x - self.direction.cos() * distance;
        let new_y = self.y - self.direction.sin() * distance;

        // Verifica si la nueva posición en el eje x no es una pared
        if !map.is_wall(new_x, self.y) {
            self.x = new_x;
        }

        // Verifica si la nueva posición en el eje y no es una pared
        if !map.is_wall(self.x, new_y) {
            self.y = new_y;
        }
    }

    /// Gira al jugador hacia la izquierda (contra las agujas del reloj).
    ///
    /// # Argumentos
    ///
    /// * `angle` - El ángulo en radianes que se debe girar a la izquierda.
    pub fn turn_left(&mut self, angle: f64) {
        self.direction -= angle;
    }

    /// Gira al jugador hacia la derecha (en el sentido de las agujas del reloj).
    ///
    /// # Argumentos
    ///
    /// * `angle` - El ángulo en radianes que se debe girar a la derecha.
    pub fn turn_right(&mut self, angle: f64) {
        self.direction += angle;
    }

    /// Permite al jugador girar en cualquier dirección con un ajuste más fino.
    ///
    /// # Argumentos
    ///
    /// * `angle` - El ángulo en radianes que se debe girar. Un valor positivo
    /// girará a la derecha y un valor negativo girará a la izquierda.
    pub fn rotate(&mut self, angle: f64) {
        self.direction += angle;
    }

    /// Ajusta el campo de visión del jugador.
    ///
    /// # Argumentos
    ///
    /// * `fov` - El nuevo campo de visión en grados.
    pub fn set_fov(&mut self, fov: f64) {
        self.fov = fov.to_radians();
    }
}
