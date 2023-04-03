use three_d::*;

type Point = Vector3<f32>;

struct Tetrahedron {
    transform: Mat4,
    colors: [Color; 4],
}

struct Meshh {
    positions: Vec<Point>,
    faces: Vec<usize>,
}

impl Tetrahedron {
    fn new(transform: Mat4) -> Self {
        Tetrahedron {
            transform: transform,
            colors: [BLACK, BLACK, BLACK, BLACK],
        }
    }
    fn mesh() -> Meshh {
        let sqrt2_over2 = 2.0_f32.sqrt() / 2.0;
        let vertices = [
            vec3(1.0, 0.0, -sqrt2_over2 * 1.0),
            vec3(-1.0, 0.0, -sqrt2_over2 * 1.0),
            vec3(0.0, 1.0, sqrt2_over2 * 1.0),
            vec3(0.0, -1.0, sqrt2_over2 * 1.0),
        ];
        let result_faces = vec![0, 1, 2, 0, 3, 1, 2, 1, 3, 0, 2, 3];
        Meshh {
            positions: vertices.into(),
            faces: result_faces,
        }
    }

    fn subdivide(&self, depth: i32) -> Vec<Tetrahedron> {
        let sqrt2_over2 = 2.0_f32.sqrt() / 2.0;
        let depth: f32 = depth as f32;
        let transforms = [
            [0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
            [1.0, 1.0, 2.0 * sqrt2_over2],
            [1.0, -1.0, 2.0 * sqrt2_over2],
        ];
        let transforms = transforms.map(|t| Vec3::from(t) * depth);
        transforms
            .map(|t| Tetrahedron::new(self.transform * (Mat4::from_translation(t.into()))))
            .into()
        // for t in transforms {
        //    Tetrahedron::new(self.transform * Mat4::from_translation(t.into()));
        // }

        // vec![
        //     Tetrahedron::new(self.transform * Mat4::from_translation(Vec3::new(0.0, 0.0, 0.0))),
        //     Tetrahedron::new(
        //         self.transform * Mat4::from_translation(Vec3::new(depth * 2.0, 0.0, 0.0)),
        //     ),
        //     Tetrahedron::new(
        //         self.transform
        //             * Mat4::from_translation(Vec3::new(
        //                 depth * 1.0,
        //                 depth * 1.0,
        //                 depth * 2.0 * sqrt2_over2,
        //             )),
        //     ),
        //     Tetrahedron::new(
        //         self.transform
        //             * Mat4::from_translation(Vec3::new(
        //                 depth * 1.0,
        //                 -1.0 * depth,
        //                 depth * 2.0 * sqrt2_over2,
        //             )),
        //     ),
        // ]
    }

    fn cpu_mesh() -> CpuMesh {
        let mesh = Self::mesh();
        let indices: Vec<u16> = mesh.faces.iter().map(|&x| x as u16).collect();

        let mut cpu_mesh = CpuMesh {
            positions: Positions::F32(mesh.positions.into()),
            indices: Indices::U16(indices),
            ..Default::default()
        };
        cpu_mesh.compute_normals();
        cpu_mesh
    }
}

const WHITE: Color = Color::new(255, 255, 255, 255);
const RED: Color = Color::new(255, 0, 0, 255);
const GREEN: Color = Color::new(0, 255, 0, 255);
const CYAN: Color = Color::new(0, 255, 255, 255);
const BLUE: Color = Color::new(0, 0, 255, 255);
const BLACK: Color = Color::new(0, 0, 0, 255);

pub fn main() {
    // Create a window (a canvas on web)
    let window = Window::new(WindowSettings {
        title: "Triangle!".to_string(),
        max_size: Some((1024, 1024)),
        ..Default::default()
    })
    .unwrap();

    // Get the graphics context from the window
    let context = window.gl();

    // Create a camera
    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 0.0, 5.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        100.0,
    );

    let mut control = OrbitControl::new(vec3(0.0, 0.0, 0.0), 0.1, 100.0);

    let mut tets = vec![Tetrahedron::new(Mat4::from_scale(1.0))];
    for iter in 0..8 {
        tets = tets.iter().flat_map(|t| t.subdivide(iter)).collect();
    }

    let ambient = AmbientLight::new(&context, 0.4, Color::WHITE);
    let directional1 = DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(-1.0, -1.0, -1.0));
    let directional2 = DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(1.0, 1.0, 1.0));

    let mut models = Vec::new();

    let mut instanced_mesh = Gm::new(
        InstancedMesh::new(&context, &Instances::default(), &Tetrahedron::cpu_mesh()),
        PhysicalMaterial::new(
            &context,
            &CpuMaterial {
                albedo: Color {
                    r: 128,
                    g: 128,
                    b: 128,
                    a: 255,
                },
                ..Default::default()
            },
        ),
    );

    instanced_mesh.set_instances(&Instances {
        transformations: tets.iter().map(|t| t.transform).collect(),
        colors: Some(tets.iter().flat_map(|t| t.colors.to_vec()).collect()),
        ..Default::default()
    });
    models.push(instanced_mesh);
    let axes = Axes::new(&context, 0.01, 10.0);
    // Start the main render loop
    window.render_loop(
        move |mut frame_input| // Begin a new frame with an updated frame input
            {
                control.handle_events(&mut camera, &mut frame_input.events);

                // Ensure the viewport matches the current window viewport which changes if the window is resized
                camera.set_viewport(frame_input.viewport);

                // Set the current transformation of the triangle
                //model.set_transformation(Mat4::from_angle_y(radians((frame_input.accumulated_time * 0.002) as f32)));


                // Get the screen render target to be able to render something on the screen
                frame_input.screen()
                    // Clear the color and depth of the screen render target
                    .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
                    // .render(&camera, &axes, &[&ambient])
                    // Render the triangle with the color material which uses the per vertex colors defined at construction
                    .render(
                        &camera, &models, &[&ambient, &directional1, &directional2],
                    );

                // Returns default frame output to end the frame
                FrameOutput::default()
            },
    );
}
