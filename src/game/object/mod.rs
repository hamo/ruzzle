use crate::game::{Asset, Direction, StatusManager, Status, World};
use crate::game::character::Player;
use crate::audio::AudioPlayer;
pub mod wall;
pub mod rock;
pub mod exit;
pub mod projector;
pub mod projectile;
pub use self::wall::Wall;
pub use self::rock::Rock;
pub use self::exit::Exit;
pub use self::projector::Projector;
pub use self::projectile::Projectile;

pub trait Object {
    fn asset(&self) -> &Asset;
    fn status_manager(&self) -> &StatusManager;
    fn attribute_manager(&mut self) -> &mut AttributeManager;
    fn update(&mut self, _now: f64, _world: &World, _audio: &mut Box<dyn AudioPlayer>) {}
    fn walk(&mut self, _direction: Direction, _world: &World) {}
    fn interact(&mut self, _player: &mut Player) {}
    fn can_move_to(&self, direction: &Direction, world: &World) -> bool {
        let next_position = self.status_manager().get_next_position_by_direction(&direction);
        if !next_position.is_in_tile_map() {
            return false;
        }
        if self.status_manager().status != Status::Idle {
            return false;
        }
        let object = world.get_object_by_position(&next_position);
        let mut has_object = false;
        if let Some(_) = object {
            has_object = true;
        }
        !has_object
    }
    fn rotate(&mut self) {}
}

pub struct AttributeManager {
    pub id: String,
    pub is_visible: bool,
    pub can_step_on: bool,
    pub is_pushable: bool,
    pub is_filler: bool,
    pub is_rotatable: bool,
    pub is_projecting: bool,
    pub is_projectile: bool,
}