extern crate nalgebra as na;

use na::Vector2;
use ncollide2d::shape::{Ball, ShapeHandle};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodySet, DefaultColliderSet, RigidBody, RigidBodyDesc,
};
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};

fn main() {
    let gravity = Vector2::y() * -9.81;
    let mut mechanical_world = DefaultMechanicalWorld::new(gravity);
    let mut geometrical_world = DefaultGeometricalWorld::new();

    let mut joint_constraints = DefaultJointConstraintSet::new();
    let mut force_generators = DefaultForceGeneratorSet::new();

    let mut bodies = DefaultBodySet::new();
    let rigid_body = RigidBodyDesc::new()
        .translation(Vector2::y() * 5.0)
        .mass(1.2)
        .build();
    let rigid_body_handle = bodies.insert(rigid_body);

    let mut colliders = DefaultColliderSet::new();
    let shape = ShapeHandle::new(Ball::new(1.5));
    let collider = ColliderDesc::new(shape).build(BodyPartHandle(rigid_body_handle, 0));
    colliders.insert(collider);

    loop {
        // Run the simulation.
        mechanical_world.step(
            &mut geometrical_world,
            &mut bodies,
            &mut colliders,
            &mut joint_constraints,
            &mut force_generators,
        );
        for (_, body) in bodies.iter() {
            if body.is::<RigidBody<f32>>() {
                let rigid_body = body.downcast_ref::<RigidBody<f32>>().unwrap();
                println!("body: {}", rigid_body.position().translation);
            }
        }
    }
}
