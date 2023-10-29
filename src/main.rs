use ascii_renderer::{Camera, Color, Fragment, FrameBuffer, Model, Triangle};
use cgmath::{frustum, Matrix4, Perspective, Point3, Vector3, Vector4};
use std::{io::stdout, time::Instant};

const WIDTH: usize = 128;
const HEIGHT: usize = 128;

fn main() {
    let mut model = Model::triangle();

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

    let mut stdout = stdout();
    let mut fragments: Vec<Fragment> = vec![];
    let mut frame_buffer = FrameBuffer::new(WIDTH, HEIGHT, &mut stdout);
    let mut primitives: Vec<Triangle> = vec![];

    let mut time = Instant::now();
    loop {
        frame_buffer.clear();

        model.rotate(Vector3::new(
            0.0,
            time.elapsed().as_millis() as f64 / 10.0,
            0.0,
        ));
        time = Instant::now();

        let mvp_matrix = proj_matrix * view_matrix * model.model_matrix();

        primitives.clear();
        fragments.clear();

        primitives.append(&mut model.polygons().to_vec());

        primitives
            .iter_mut()
            .map(|polygon| vertex_shader(polygon, &mvp_matrix))
            .for_each(|primitive| rasterizer(primitive, &mut fragments));

        fragments.iter_mut().for_each(|fragment| {
            fragment_shader(fragment);

            frame_buffer.set_pixel(*fragment);
        });

        frame_buffer.print().expect("error");
        println!("{}", time.elapsed().as_millis());

        // thread::sleep(Duration::from_millis(100));
    }
}

fn vertex_shader<'a>(triangle: &'a mut Triangle, mvp_matrix: &'a Matrix4<f64>) -> &'a mut Triangle {
    for vertex in &mut triangle.vertices {
        let v = Vector4::new(vertex.pos.x, vertex.pos.y, vertex.pos.z, 1.0);
        let clip_space = mvp_matrix * v; // to clip space
        let point = Point3::from_homogeneous(clip_space); // to NDC

        vertex.pos = point;
    }
    triangle
}

fn rasterizer(triangle: &mut Triangle, fragments: &mut Vec<Fragment>) {
    // viewport transform
    for vertex in &mut triangle.vertices {
        vertex.pos.x = ((vertex.pos.x + 1.0) / 2.0) * (WIDTH as f64);
        vertex.pos.y = ((vertex.pos.y + 1.0) / 2.0) * (HEIGHT as f64);
        vertex.pos.z = (vertex.pos.z + 1.0) / 2.0;
    }

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let p = &Point3::new(j as f64 + 0.5, i as f64 + 0.5, 0.0);

            let v0 = &triangle.vertices[0];
            let v1 = &triangle.vertices[1];
            let v2 = &triangle.vertices[2];

            let area = edge(&v0.pos, &v1.pos, &v2.pos);
            let mut w0: f64 = edge(&v1.pos, &v2.pos, p);
            let mut w1 = edge(&v2.pos, &v0.pos, p);
            let mut w2 = edge(&v0.pos, &v1.pos, p);

            if w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0 {
                w0 /= area;
                w1 /= area;
                w2 /= area;

                fragments.push(Fragment {
                    color: Color::round(v0.color * w0 + v1.color * w1 + v2.color * w2),
                    x: j,
                    y: i,
                    z: 0.5,
                })
            }
        }
    }
}

fn edge(a: &Point3<f64>, b: &Point3<f64>, c: &Point3<f64>) -> f64 {
    (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
}

fn fragment_shader(fragment: &mut Fragment) -> &mut Fragment {
    fragment
}
