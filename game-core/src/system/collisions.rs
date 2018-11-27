use amethyst_rhusics::{
    rhusics_core::{
        ContactEvent,
        Pose
    },
};
use amethyst::{
    core::{Transform},
    ecs::{Entity, Join, Read, ReadStorage, Resources, System, SystemData, WriteStorage},
    shrev::{ReaderId, EventChannel},
};
use amethyst::core::cgmath::Point2;
use crate::component::{Player, Enemy, Projectile};

#[derive(Default)]
pub struct Collisions {
    contact_reader: Option<ReaderId<ContactEvent<Entity, Point2<f32>>>>,
}

impl<'s> System<'s> for Collisions {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, Projectile>,
        WriteStorage<'s, Transform>,
        Read<'s, EventChannel<ContactEvent<Entity, Point2<f32>>>>
    );

    fn run(
        &mut self,
        (
            players,
            enemies,
            projectiles,
            tranforms,
            contacts
        ): Self::SystemData,
    ) {
        for contact in contacts.read(&mut self.contact_reader.as_mut().unwrap()) {
            println!("got a collision!");
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        println!("Setting up collision detection");

        self.contact_reader = Some(
            res.fetch_mut::<EventChannel<ContactEvent<Entity, Point2<f32>>>>()
                .register_reader(),
        );
    }
}