use crate::player::Player;
use crate::map::Map;

/// Lanza un rayo desde la posición del jugador y calcula la distancia hasta la primera pared que encuentra.
///
/// # Arguments
/// * `map` - Referencia al mapa del juego.
/// * `player` - Referencia al jugador.
/// * `angle_offset` - Desplazamiento angular para calcular la dirección del rayo.
///
/// # Returns
/// * `(f64, bool)` - Distancia perpendicular a la pared y un booleano indicando si la pared es horizontal.
pub fn cast_ray(map: &Map, player: &Player, angle_offset: f64) -> (f64, bool) {
    let ray_angle = player.direction + angle_offset;

    // Dirección del rayo
    let ray_dir_x = ray_angle.cos();
    let ray_dir_y = ray_angle.sin();

    // Posiciones iniciales en la rejilla
    let mut map_x = player.x.floor() as isize;
    let mut map_y = player.y.floor() as isize;

    // Distancia recorrida por el rayo desde una pared a la siguiente
    let delta_dist_x = if ray_dir_x != 0.0 {
        (1.0 / ray_dir_x).abs()
    } else {
        f64::INFINITY
    };
    let delta_dist_y = if ray_dir_y != 0.0 {
        (1.0 / ray_dir_y).abs()
    } else {
        f64::INFINITY
    };

    // Variables de paso
    let (step_x, mut side_dist_x) = if ray_dir_x < 0.0 {
        (-1, (player.x - map_x as f64) * delta_dist_x)
    } else {
        (1, (map_x as f64 + 1.0 - player.x) * delta_dist_x)
    };

    let (step_y, mut side_dist_y) = if ray_dir_y < 0.0 {
        (-1, (player.y - map_y as f64) * delta_dist_y)
    } else {
        (1, (map_y as f64 + 1.0 - player.y) * delta_dist_y)
    };

    let mut hit = false; // Si el rayo ha golpeado una pared
    let mut side = 0; // 0 para vertical, 1 para horizontal

    // Bucle para recorrer el mapa
    while !hit {
        // Saltar al siguiente cuadrado
        if side_dist_x < side_dist_y {
            side_dist_x += delta_dist_x;
            map_x += step_x;
            side = 0;
        } else {
            side_dist_y += delta_dist_y;
            map_y += step_y;
            side = 1;
        }

        // Comprobar si el rayo ha golpeado una pared
        if map.is_wall(map_x as f64, map_y as f64) {
            hit = true;
        }
    }

    // Calcular distancia perpendicular a la pared para evitar distorsión
    let perp_wall_dist = if side == 0 {
        (map_x as f64 - player.x + (1 - step_x) as f64 / 2.0) / ray_dir_x
    } else {
        (map_y as f64 - player.y + (1 - step_y) as f64 / 2.0) / ray_dir_y
    };

    (perp_wall_dist, side == 1)
}
