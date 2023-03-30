use cgmath::{Decomposed, Vector3, Quaternion, Rotation3, Rad};
use collision::{primitive::Cuboid, algorithm::minkowski::GJK3};

fn transform_3d(
    x: f32,
    y: f32,
    z: f32,
    angle_z: f32,
) -> Decomposed<Vector3<f32>, Quaternion<f32>> {
    Decomposed {
        disp: Vector3::new(x, y, z),
        rot: Quaternion::from_angle_z(Rad(angle_z)),
        scale: 1.,
    }
}

fn main() {
    println!("Creating Cube with a side length of 10 at pos: [15, 0, 0].");
    let left = Cuboid::new(10., 10., 10.);
    let left_transform = transform_3d(15., 0., 0., 0.);

    println!("Creating Cube with a side length of 10 at pos: [7, 2, 0].");
    let right = Cuboid::new(10., 10., 10.);
    let right_transform = transform_3d(7., 2., 0., 0.);

    let gjk = GJK3::new();

    let distance = gjk.distance(&left, &left_transform, &right, &right_transform);
    println!("Distance is {:?} therfore the Cubes are colliding.", distance);

    println!("Setting pos of cube 2 to [1, 0, 0]");
    let right_transform = transform_3d(1., 0., 0., 0.);

    let distance2 = gjk.distance(&left, &left_transform, &right, &right_transform);
    
    println!("Distance is {:?} therfore the Cubes are not colliding.", distance2);
}