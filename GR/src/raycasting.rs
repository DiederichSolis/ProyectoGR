use glam::{U16Vec2, Vec2}; // Importa tipos de vectores desde la biblioteca 'glam' para trabajar con coordenadas 2D.

/// Estructura que representa el sistema de ray casting para un mundo en 2D.
pub struct RayCasting {
    pub world: Vec<Vec<u8>>, // Mapa 2D del mundo, donde cada valor u8 representa un tipo de bloque.
    pub pos: Vec2,           // Posición actual del jugador o cámara en el mundo.
    pub dir: Vec2,           // Dirección en la que está mirando el jugador o la cámara.
    pub plane: Vec2,         // Plano perpendicular a la dirección de la cámara, usado para calcular la proyección 3D.
}

/// Implementación de métodos para la estructura `RayCasting`.
impl RayCasting {
    /// Método que realiza el cálculo del ray casting y devuelve las líneas que representan las paredes vistas por la cámara.
    ///
    /// # Argumentos
    ///
    /// * `w` - Ancho de la pantalla.
    /// * `h` - Alto de la pantalla.
    ///
    /// # Retorna
    ///
    /// Un vector de tuplas, donde cada tupla contiene:
    /// - Las coordenadas de inicio y fin de la línea en la pantalla.
    /// - El tipo de bloque encontrado en el mundo.
    /// - Un booleano que indica si la pared fue golpeada por el lado X o Y.
    pub fn lines(&mut self, w: u16, h: u16) -> Vec<(U16Vec2, U16Vec2, u8, bool)> {
        let mut lines = Vec::with_capacity(w as usize); // Crea un vector para almacenar las líneas calculadas.
        
        for x in 0..w {
            // Calcula la dirección del rayo para cada columna de la pantalla.
            let ray_dir = self.dir + self.plane * Vec2::splat(2.0 * x as f32 / w as f32 - 1.0);
            let mut map_x = self.pos.x as usize; // Coordenada X en el mapa.
            let mut map_y = self.pos.y as usize; // Coordenada Y en el mapa.

            // Inicializa la distancia al siguiente lado de la cuadrícula en X e Y.
            let mut side_dist = Vec2::ZERO;

            // Calcula la distancia desde un borde del mapa al siguiente en X e Y.
            let delta_dist_x = if ray_dir.x == 0.0 {
                1e30
            } else {
                (1.0 / ray_dir.x).abs()
            };

            let delta_dist_y = if ray_dir.y == 0.0 {
                1e30
            } else {
                (1.0 / ray_dir.y).abs()
            };

            let mut hit = 0;  // Variable para determinar si se ha golpeado una pared.
            let mut side = 0; // Variable para determinar si se golpeó el lado X o Y de una pared.
            let step_x: i8;   // Dirección en la que el rayo se moverá en X.
            let step_y: i8;   // Dirección en la que el rayo se moverá en Y.

            // Determina la dirección y la distancia inicial al siguiente lado de la cuadrícula en X.
            if ray_dir.x < 0.0 {
                step_x = -1;
                side_dist.x = (self.pos.x - map_x as f32) * delta_dist_x;
            } else {
                step_x = 1;
                side_dist.x = (map_x as f32 + 1.0 - self.pos.x) * delta_dist_x;
            }

            // Determina la dirección y la distancia inicial al siguiente lado de la cuadrícula en Y.
            if ray_dir.y < 0.0 {
                step_y = -1;
                side_dist.y = (self.pos.y - map_y as f32) * delta_dist_y;
            } else {
                step_y = 1;
                side_dist.y = (map_y as f32 + 1.0 - self.pos.y) * delta_dist_y;
            }

            // Bucle para avanzar el rayo hasta que golpee una pared.
            while hit == 0 {
                // Avanza el rayo en la dirección de menor distancia.
                if side_dist.x < side_dist.y {
                    side_dist.x += delta_dist_x;
                    map_x = (map_x as i8 + step_x) as usize;
                    side = 0;
                } else {
                    side_dist.y += delta_dist_y;
                    map_y = (map_y as i8 + step_y) as usize;
                    side = 1;
                }

                // Verifica si el rayo ha golpeado una pared.
                if self.world[map_y][map_x] > 0 {
                    hit = 1;
                }
            }

            // Calcula la distancia perpendicular desde la cámara a la pared.
            let perp_wall_dist = if side == 0 {
                side_dist.x - delta_dist_x
            } else {
                side_dist.y - delta_dist_y
            };

            // Calcula la altura de la línea que representa la pared en la pantalla.
            let line_height = h as f32 / perp_wall_dist;
            let mut draw_start = -line_height / 2.0 + h as f32 / 2.0;

            // Ajusta la posición de inicio si está fuera de la pantalla.
            if draw_start < 0.0 {
                draw_start = 0.0;
            }

            let mut draw_end = line_height / 2.0 + h as f32 / 2.0;

            // Ajusta la posición de fin si está fuera de la pantalla.
            if draw_end >= h as f32 {
                draw_end = h as f32 - 1.0;
            }

            // Almacena la línea calculada en el vector `lines`.
            lines.push((
                U16Vec2::new(x, draw_start as u16), // Coordenadas de inicio de la línea.
                U16Vec2::new(x, draw_end as u16),   // Coordenadas de fin de la línea.
                self.world[map_y][map_x],           // Tipo de bloque golpeado.
                side == 1,                          // Indica si se golpeó por el lado Y.
            ));
        }

        lines // Retorna el vector con todas las líneas calculadas.
    }

    /// Método que permite mover y rotar la cámara (jugador) en el mundo.
    ///
    /// # Argumentos
    ///
    /// * `move_factor` - Factor que determina la distancia de movimiento en la dirección de la cámara.
    /// * `rotation_factor` - Factor que determina el ángulo de rotación de la cámara.
    pub fn transform_cam(&mut self, move_factor: f32, rotation_factor: f32) {
        // Movimiento de la cámara
        if move_factor != 0.0 {
            // Verifica si la posición a la que la cámara se movería en X está libre.
            if self.world[self.pos.y as usize][(self.pos + self.dir * move_factor).x as usize] == 0 {
                self.pos.x += (self.dir * move_factor).x; // Mueve la cámara en X.
            }

            // Verifica si la posición a la que la cámara se movería en Y está libre.
            if self.world[(self.pos + self.dir * move_factor).y as usize][self.pos.x as usize] == 0 {
                self.pos.y += (self.dir * move_factor).y; // Mueve la cámara en Y.
            }
        }

        // Rotación de la cámara
        if rotation_factor != 0.0 {
            let old_dir_x = self.dir.x; // Guarda la dirección X actual para cálculos posteriores.

            // Aplica la matriz de rotación a la dirección de la cámara.
            self.dir.x = self.dir.x * rotation_factor.cos() - self.dir.y * rotation_factor.sin();
            self.dir.y = old_dir_x * rotation_factor.sin() + self.dir.y * rotation_factor.cos();

            let old_plane_x = self.plane.x; // Guarda el plano X actual para cálculos posteriores.

            // Aplica la matriz de rotación al plano de la cámara.
            self.plane.x = self.plane.x * rotation_factor.cos() - self.plane.y * rotation_factor.sin();
            self.plane.y = old_plane_x * rotation_factor.sin() + self.plane.y * rotation_factor.cos();
        }
    }
}
