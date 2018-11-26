use amethyst_rhusics::rhusics_core::physics3d::Velocity3;
use amethyst::core::{Transform, Time};
use amethyst::{
    ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage}
};
use amethyst::core::cgmath::Point2;
use crate::component::{Player, Enemy, Projectile};

pub struct Collisions;
impl<'s> System<'s> for Collisions {
    type SystemData = (
        WriteStorage<'s, Player>
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, Projectile>,
        WriteStorage<'s, Transform>
    );

    fn run(
        &mut self,
        (
            players,
            enemies,
            projectiles,
            tranforms
        ): Self::SystemData,
    ) {
        
    }
}