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
            transform,
            colors: [BLACK, RED, GREEN, BLUE],
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

    // fn subdivide(&self, depth: i32) -> Vec<Tetrahedron> {
    //     let sqrt2_over2 = 2.0_f32.sqrt() / 2.0;
    //     let depth: f32 = depth as f32;

    //     let translation_coords = [
    //         [0.0, 0.0, 0.0],
    //         [2.0, 0.0, 0.0],
    //         [1.0, 1.0, 2.0 * sqrt2_over2],
    //         [1.0, -1.0, 2.0 * sqrt2_over2],
    //     ];

    //     // let translation_coords = [
    //     //     [0.0, 0.0, 0.0],
    //     //     [2.0 * depth, 0.0, 0.0],
    //     //     [1.0 * depth, 1.0 * depth, depth * 2.0 * sqrt2_over2],
    //     //     [depth * 1.0, depth * -1.0, depth * 2.0 * sqrt2_over2],
    //     // ];

    //     let translations = translation_coords.map(|tc| Mat4::from_translation(tc.into()));
    //     // let transforms = Mat4::from_translation(t.into());
    //     let half = 0.5_f32;
    //     translations
    //         // .map(|t| Tetrahedron::new(self.transform * ( Mat4::from_scale(half.powf(depth)))))
    //         .map(|t| Tetrahedron::new(self.transform * t))
    //         .into()
    // }

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

    fn transformations(transform: Mat4, iterations: i32) -> Instances {
        let mut transforms = vec![transform];
        for iter in 0..iterations {
            transforms = transforms.iter().flat_map(|t| subdivide(*t)).collect();
        }
        Instances {
            transformations: transforms.iter().map(|t| *t).collect(),
            // colors: Some(
            //     transforms
            //         .iter()
            //         .flat_map(|t| self.colors.to_vec())
            //         .collect(),
            // ),
            ..Default::default()
        }
    }
}

fn subdivide(input: Mat4) -> Vec<Mat4> {
    let sqrt2_over2 = 2.0_f32.sqrt() / 2.0;
    let translation_coords = [
        [0.0, 0.0, 0.0],
        [2.0, 0.0, 0.0],
        [1.0, 1.0, 2.0 * sqrt2_over2],
        [1.0, -1.0, 2.0 * sqrt2_over2],
    ];

    translation_coords
        .map(|tc| Mat4::from_translation(tc.into()))
        .into()
}

fn subdivide_anim(input: Mat4) -> Vec<Animatrix> {
    subdivide(input)
        .iter()
        .map(|m| Animatrix {
            t: 0.0,
            vel: 0.0,
            from: input,
            to: *m,
        })
        .collect()
}

struct Animatrix {
    t: f32,
    vel: f32,
    from: Mat4,
    to: Mat4,
}

impl Animatrix {
    fn step(&mut self, millis: i32) -> Mat4 {
        if self.t == 1.0 {
            return self.to;
        }
        self.t = self.t + millis as f32 * self.vel;
        if self.t > 1.0 {
            self.t = 1.0
        }
        self.from.lerp(self.to, self.t)
    }
}

struct Fractal {
    iterations: i32,
    anims: Vec<Animatrix>,
    tetrahedron: Gm<InstancedMesh, PhysicalMaterial>,
}

impl Fractal {
    fn new(context: &Context) -> Fractal {
        Fractal {
            iterations: 0,
            anims: Vec::new(),
            tetrahedron: Gm::new(
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
            ),
        }
    }
    fn update_iterations(&mut self, new_iter: i32) {
        let mut transforms = vec![self.tetrahedron.transformation()];
        for _ in 0..new_iter {
            transforms = transforms.iter().flat_map(|t| subdivide(*t)).collect();
        }
        let instances = Instances {
            transformations: transforms.iter().map(|t| *t).collect(),
            ..Default::default()
        };
        // colors: Some(
        self.iterations = new_iter;
        self.tetrahedron.set_instances(&instances);
    }
    fn update_animation(&self, t: f64) {}
    fn increment_iterations(&mut self) {
        self.update_iterations(self.iterations + 1)
    }
    fn decrement_iterations(&mut self) {
        self.update_iterations(self.iterations - 1)
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

    let ambient = AmbientLight::new(&context, 0.4, Color::WHITE);
    let directional1 = DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(-1.0, -1.0, -1.0));
    let directional2 = DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(1.0, 1.0, 1.0));

    // let mut models = Vec::new();

    let mut frac = Fractal::new(&context);

    // models.push(instanced_mesh);
    let axes = Axes::new(&context, 0.01, 10.0);
    let mut exit = false;
    // Start the main render loop
    window.render_loop(
        move |mut frame_input| // Begin a new frame with an updated frame input
            {
                control.handle_events(&mut camera, &mut frame_input.events);

                for event in frame_input.events.iter(){
                    if let Event::KeyPress {kind, .. } = event {
                        if *kind == Key::ArrowUp  {
                            frac.increment_iterations()
                        }
                        if *kind == Key::ArrowDown  {
                            frac.decrement_iterations()
                        }
                        if *kind == Key::Escape  {
                            exit = true;
                        }
                    }
                };
            frac.update_animation(frame_input.elapsed_time);
                // Ensure the viewport matches the current window viewport which changes if the window is resized
                camera.set_viewport(frame_input.viewport);

                // Set the current transformation of the triangle
                // instanced_mesh.set_transformation(Mat4::from_angle_y(radians((frame_input.accumulated_time * 0.002) as f32)));

                // Get the screen render target to be able to render something on the screen
                frame_input.screen()
                    // Clear the color and depth of the screen render target
                    .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
                    .render(&camera, &axes, &[&ambient])
                    // Render the triangle with the color material which uses the per vertex colors defined at construction
                    .render(
                        &camera, &frac.tetrahedron, &[&ambient, &directional1, &directional2],
                    );

                if exit{
                    return FrameOutput{
                        exit: true,
                        ..Default::default()
                    }
                };
                // Returns default frame output to end the frame
                FrameOutput::default()
            },
    );
}
