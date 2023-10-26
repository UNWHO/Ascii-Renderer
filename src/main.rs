use std::{thread, time::Duration};

use ascii_renderer::{Camera, Color, Fragment, Model, Triangle, Vertex};
use cgmath::{frustum, Deg, Matrix4, Perspective, Point3, SquareMatrix, Vector3, Vector4};

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

fn main() {
    let triangle = Triangle::new([
        Vertex::new(Point3::new(0.0, 0.5, 0.0), Color::new(255, 0, 0)),
        Vertex::new(Point3::new(-0.5, -0.5, 0.0), Color::new(0, 255, 0)),
        Vertex::new(Point3::new(0.5, -0.5, 0.0), Color::new(0, 0, 255)),
    ]);

    let mut model = Model::new();
    model.add_polygon(triangle);

    let mut camera = Camera::new();
    camera
        .move_to(&Point3::new(0.0, 0.0, -1.0))
        .rotate(&Point3::new(0.0, 0.0, 0.0), &Vector3::new(0.0, 1.0, 0.0));

    let view_matrix: Matrix4<f64> = Matrix4::look_at_lh(camera.pos, camera.look_at, camera.up);

    let aspect_ratio = 1.0;
    let perspective = Perspective::<f64> {
        near: 0.1,
        far: 100.0,
        top: 0.1,
        bottom: -0.1,
        right: 0.1 * aspect_ratio,
        left: -0.1 * aspect_ratio,
    };
    let proj_matrix: Matrix4<f64> = frustum(
        perspective.left,
        perspective.right,
        perspective.bottom,
        perspective.top,
        perspective.near,
        perspective.far,
    );

    let mut frame_buffer = [[' '; WIDTH]; HEIGHT];

    loop {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                frame_buffer[i][j] = ' ';
            }
        }

        model.rotate(Vector3::new(0.0, 0.0, 1.0));

        let model_matrix: Matrix4<f64> = Matrix4::identity()
            * Matrix4::from_translation(model.pos)
            * Matrix4::from_angle_x(Deg { 0: model.rot.x })
            * Matrix4::from_angle_y(Deg { 0: model.rot.y })
            * Matrix4::from_angle_z(Deg { 0: model.rot.z })
            * Matrix4::from_nonuniform_scale(model.scale.x, model.scale.y, model.scale.z);
        let mvp_matrix = proj_matrix * view_matrix * model_matrix;

        model
            .polycons
            .iter_mut()
            .map(|polygon| vertex_shader(polygon.clone(), &mvp_matrix))
            .map(|primitive| rasterizer(primitive))
            .flatten()
            .for_each(|mut fragment| {
                fragment_shader(&mut fragment);

                frame_buffer[fragment.y as usize][fragment.x as usize] = '*';
            });

        // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        print!("\x1B[2J\x1B[1;1H");
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                print!("{}", frame_buffer[i][j]);
            }
            println!();
        }
        thread::sleep(Duration::from_millis(10));
    }
}

fn vertex_shader<'a>(mut triangle: Triangle, mvp_matrix: &'a Matrix4<f64>) -> Triangle {
    for vertex in &mut triangle.vertices {
        let v = Vector4::new(vertex.pos.x, vertex.pos.y, vertex.pos.z, 1.0);
        let clip_space = mvp_matrix * v; // to clip space
        let point = Point3::from_homogeneous(clip_space); // to NDC

        vertex.pos = point;
    }
    triangle
}

fn rasterizer(mut triangle: Triangle) -> Vec<Fragment> {
    // viewport transform
    for vertex in &mut triangle.vertices {
        vertex.pos.x = ((vertex.pos.x + 1.0) / 2.0) * (WIDTH as f64);
        vertex.pos.y = ((vertex.pos.y + 1.0) / 2.0) * (HEIGHT as f64);
        vertex.pos.z = (vertex.pos.z + 1.0) / 2.0;
    }

    let mut fragments: Vec<Fragment> = Vec::new();
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let x = j as f64 + 0.5;
            let y = i as f64 + 0.5;

            let mut inside = true;
            inside = inside
                && edge(
                    &Point3::new(x, y, 0.0),
                    &triangle.vertices[0].pos,
                    &triangle.vertices[1].pos,
                );
            inside = inside
                && edge(
                    &Point3::new(x, y, 0.0),
                    &triangle.vertices[1].pos,
                    &triangle.vertices[2].pos,
                );
            inside = inside
                && edge(
                    &Point3::new(x, y, 0.0),
                    &triangle.vertices[2].pos,
                    &triangle.vertices[0].pos,
                );
            if inside == false {
                continue;
            }

            fragments.push(Fragment {
                color: Color::white(),
                x: j as i32,
                y: i as i32,
                z: 0.5,
            })
        }
    }
    fragments
}

fn edge(p: &Point3<f64>, a: &Point3<f64>, b: &Point3<f64>) -> bool {
    let value = (a.x - b.x) * (p.y - a.y) - (a.y - b.y) * (p.x - a.x);

    value <= 0.0
}

fn fragment_shader(fragment: &mut Fragment) -> &mut Fragment {
    fragment
}
