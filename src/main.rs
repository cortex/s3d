use three_d::*;

type Point = Vector3<f32>;

const WHITE: Color = Color::new(255, 255, 255, 255);
const RED: Color = Color::new(255, 0, 0, 255);
const GREEN: Color = Color::new(0, 255, 0, 255);
const BLUE: Color = Color::new(0, 0, 255, 255);


fn tetra(_origin: Vec3) -> (Vec<Point>, Vec<u16>, Vec<Color>) {
    let a = (8.0 / 9_f32).sqrt();
    let b = (2.0 / 9_f32).sqrt();
    let c = (2.0 / 3_f32).sqrt();
    let d = 1.0 / 3_f32;

    let corners: Vec<Point> = vec![
        // vec3(1.0, 1.0, 1.0),
        // vec3(1.0, -1.0, -1.0),
        // vec3(-1.0, 1.0, -1.0),
        // vec3(-1.0, -1.0, 1.0),
        vec3(0.0, 0.0, 1.0),
        vec3(a, 0.0, -d),
        vec3(-b, c, -d),
        vec3(-b, -c, -d),
    ];

    //(corners, vec![0, 1, 2])
    //(corners, vec![0, 3, 1])
    (
        corners,
        vec![0, 1, 2, 0, 3, 1, 0, 2, 3, 1, 2, 3],
        vec![RED, GREEN, BLUE, WHITE],
    )
}

fn subdivide(positions: &mut Vec<Point>, indices: &mut Vec<u16>, colors: &mut Vec<Color>) {
    let old_indices = indices.clone();
    indices.clear();
    for face in old_indices.chunks(3) {
        let (mut new_pos, mut new_indices) = subdivide_face(positions, face.into(), colors);
        positions.append(&mut new_pos);
        indices.append(&mut new_indices);
    }
}

// Return new positions, new indices
fn subdivide_face(
    positions: &mut Vec<Point>,
    indices: Vec<u16>,
    colors: &mut Vec<Color>,
) -> (Vec<Point>, Vec<u16>) {
    let l = positions.len() as u16;
    let mut new_pos = Vec::new();
    let segments: Vec<u8> = vec![0, 1, 1, 2, 2, 0];

    for segment in segments.chunks(2) {
        new_pos.push(midpoint(
            positions[usize::from(indices[usize::from(segment[0])])],
            positions[usize::from(indices[usize::from(segment[1])])],
        ));
        colors.push(WHITE);
    }

    let a = indices[0];
    let b = indices[1];
    let c = indices[2];

    let d = l + 0;
    let e = l + 1;
    let f = l + 2;

    let new_indices = vec![a, d, f, d, b, e, f, e, c];
    //let new_indices = vec![d, e, f];

    (new_pos, new_indices)
}

fn midpoint(p1: Point, p2: Point) -> Point {
    Vector3::new(
        (p1.x + p2.x) / 2.0,
        (p1.y + p2.y) / 2.0,
        (p1.z + p2.z) / 2.0,
    )
}

pub fn main() {
    // Create a window (a canvas on web)
    let window = Window::new(WindowSettings {
        title: "Triangle!".to_string(),
        max_size: Some((1280, 720)),
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
        10.0,
    );

    let mut control = OrbitControl::new(vec3(0.0, 0.0, 0.0), 1.0, 1000.0);

    let (mut positions, mut indices, mut colors) = tetra(vec3(0.0, 0.0, 0.0));
    for _ in 0..8{
    subdivide(&mut positions, &mut indices, &mut colors);
    }

    let cpu_mesh = CpuMesh {
        positions: Positions::F32(positions),
        colors: Some(colors),
        indices: Indices::U16(indices),
        ..Default::default()
    };

    let ambient = AmbientLight::new(&context, 0.4, Color::WHITE);
    let directional1 = DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(-1.0, -1.0, -1.0));
    let directional2 = DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(1.0, 1.0, 1.0));


    // Construct a model, with a default color material, thereby transferring the mesh data to the GPU
    let mut model = Gm::new(Mesh::new(&context, &cpu_mesh), ColorMaterial::default());

    // Start the main render loop
    window.render_loop(move |frame_input: FrameInput| // Begin a new frame with an updated frame input
    {
        // Ensure the viewport matches the current window viewport which changes if the window is resized
        camera.set_viewport(frame_input.viewport);

        // Set the current transformation of the triangle
        model.set_transformation(Mat4::from_angle_y(radians((frame_input.accumulated_time * 0.002) as f32)));

        // Get the screen render target to be able to render something on the screen
        frame_input.screen()
            // Clear the color and depth of the screen render target
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            // Render the triangle with the color material which uses the per vertex colors defined at construction
            .render(
                &camera, &model,                 
                    &[&ambient, &directional1, &directional2],
            );

        // Returns default frame output to end the frame
        FrameOutput::default()
    });
}
