extern crate nalgebra as na;

use na::Vector2;
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet, RigidBodyDesc};
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};

fn main() {
    let gravity = Vector2::y() * -9.81;
    let mut mechanical_world = DefaultMechanicalWorld::new(gravity);
    let mut geometrical_world = DefaultGeometricalWorld::new();

    let mut colliders = DefaultColliderSet::new();
    let mut joint_constraints = DefaultJointConstraintSet::new();
    let mut force_generators = DefaultForceGeneratorSet::new();

    let mut bodies = DefaultBodySet::new();
    let rigid_body = RigidBodyDesc::new()
        .translation(Vector2::y() * 5.0)
        .mass(1.2)
        .build();
    bodies.insert(rigid_body);

    loop {
        // Run the simulation.
        mechanical_world.step(
            &mut geometrical_world,
            &mut bodies,
            &mut colliders,
            &mut joint_constraints,
            &mut force_generators,
        )
    }
}
