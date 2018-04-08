use std::f64;
use std::f64::consts::PI;

use env_logger;
use ndarray::Array1;
use piston_window as pw;
use piston_window::{ButtonEvent, FocusEvent, UpdateEvent};

use input_state::InputState;
use polytopes::{Vector3d, Edge, Polytope, Cube};


#[derive(Debug)]
struct DemoModel {
    polytope: Box<Polytope>,
}

impl DemoModel {
    pub fn new() -> DemoModel {
        let offset = (0.0, 0.0, 5.0);
        let rotation = (0.0, 0.0, 0.0);
        let scale = 1.0;
        let polytope = Box::new(Cube { offset, rotation, scale });
        DemoModel { polytope }
    }

    pub fn update(&mut self, dt: f64, input_state: InputState) {
        trace!("Updating model");

        let mut dx = 0.0;
        let mut dy = 0.0;
        let mut dz = 0.0;

        self.polytope.rotate((0.0007 * PI, 0.002 * PI, 0.0009 * PI));

        dy += 10.0 * dt * match input_state.up { pw::ButtonState::Press => 1.0, _ => 0.0 };
        dy -= 10.0 * dt * match input_state.down { pw::ButtonState::Press => 1.0, _ => 0.0 };
        dx -= 10.0 * dt * match input_state.left { pw::ButtonState::Press => 1.0, _ => 0.0 };
        dx += 10.0 * dt * match input_state.right { pw::ButtonState::Press => 1.0, _ => 0.0 };
        dz += 10.0 * dt * match input_state.front { pw::ButtonState::Press => 1.0, _ => 0.0 };
        dz -= 10.0 * dt * match input_state.back { pw::ButtonState::Press => 1.0, _ => 0.0 };

        self.polytope.shift((dx, dy, dz));
    }
}


struct DemoView {
    input_state: InputState,
    window: pw::PistonWindow,
}

impl DemoView {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;
    const COORDS_SCALE: f64 = 400.0;

    pub fn new() -> DemoView {
        let input_state = InputState::new();

        let window =
            pw::WindowSettings::new("threedy", [Self::WIDTH, Self::HEIGHT])
            .exit_on_esc(true)
            .build()
            .expect("window init failed");

        DemoView { input_state, window }
    }

    pub fn input_state(&self) -> InputState {
        self.input_state
    }

    pub fn next_event(&mut self) -> Option<pw::Event> {
        self.window.next()
    }

    pub fn handle_render_event(&mut self, event: &pw::Event, model: &DemoModel) {
        self.window.draw_2d(event, |context, graphics| {
            Self::render(context, graphics, model);
        });
    }

    pub fn handle_button_event(&mut self, button_args: &pw::ButtonArgs) {
        debug!("Handling button event: {:?}", button_args);

        match button_args.button {
            pw::Button::Keyboard(pw::Key::W) => {
                self.input_state.front = button_args.state;
            },
            pw::Button::Keyboard(pw::Key::A) => {
                self.input_state.left = button_args.state;
            },
            pw::Button::Keyboard(pw::Key::R) => {
                self.input_state.up = button_args.state;
            },
            pw::Button::Keyboard(pw::Key::F) => {
                self.input_state.down = button_args.state;
            },
            pw::Button::Keyboard(pw::Key::S) => {
                self.input_state.back = button_args.state;
            },
            pw::Button::Keyboard(pw::Key::D) => {
                self.input_state.right = button_args.state;
            },
            _ => {},
        }
    }

    pub fn handle_focus_event(&mut self, _focused: bool) {
        debug!("Handling focus event");

        self.input_state.reset();
    }

    fn render(context: pw::Context, graphics: &mut pw::G2d, model: &DemoModel) {
        trace!("Rendering");

        let white = [1.0; 4];
        pw::clear(white, graphics);

        Self::render_polytope(context, graphics, &*model.polytope);
    }

    fn render_polytope(context: pw::Context, graphics: &mut pw::G2d, polytope: &Polytope) {
        let scale = polytope.get_scale();
        let scale_matrix = array![
            [scale, 0.0, 0.0, 0.0],
            [0.0, scale, 0.0, 0.0],
            [0.0, 0.0, scale, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        // TODO
        let (angle_x, angle_y, angle_z) = polytope.get_rotation();
        let rotation_x_matrix = array![
            [1.0, 0.0, 0.0, 0.0],
            [0.0, angle_x.cos(), -angle_x.sin(), 0.0],
            [0.0, angle_x.sin(), angle_x.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let rotation_y_matrix = array![
            [angle_y.cos(), 0.0, angle_y.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-angle_y.sin(), 0.0, angle_y.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let rotation_z_matrix = array![
            [angle_z.cos(), -angle_z.sin(), 0.0, 0.0],
            [angle_z.sin(), angle_z.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let rotation_matrix =
            rotation_z_matrix
            .dot(&rotation_y_matrix)
            .dot(&rotation_x_matrix);

        let (offset_x, offset_y, offset_z) = polytope.get_offset();
        let offset_matrix = array![
            [1.0, 0.0, 0.0, offset_x],
            [0.0, 1.0, 0.0, offset_y],
            [0.0, 0.0, 1.0, offset_z],
            [0.0, 0.0, 0.0, 1.0],
        ];

        let model_matrix = offset_matrix.dot(&rotation_matrix).dot(&scale_matrix);

        // TODO
        let view_matrix = array![
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        let fov = 90.0;
        let near = 0.1;
        let far = 100.0;
        let top = near * (fov / 180.0 * PI / 2.0).tan();
        let bottom = -top;
        let right = top;
        let left = -right;
        let projection_matrix = array![
            [2.0 * near / (right - left), 0.0, (right + left) / (right - left), 0.0],
            [0.0, 2.0 * near / (top - bottom), (top + bottom) / (top - bottom), 0.0],
            [0.0, 0.0, -(far + near) / (far - near), -(2.0 * far * near) / (far - near)],
            [0.0, 0.0, 1.0, 0.0],
        ];
//        let projection_matrix = array![
//            [1.0, 0.0, 0.0, 0.0],
//            [0.0, 1.0, 0.0, 0.0],
//            [0.0, 0.0, 1.0, 0.0],
//            [0.0, 0.0, 0.0, 1.0],
//        ];
//        let projection_matrix = array![
//            [1.0, 0.0, -1.0, 0.0],
//            [0.0, 1.0, -1.0, 0.0],
//            [0.0, 0.0, 1.0, 0.0],
//            [0.0, 0.0, 0.0, 1.0],
//        ];
        println!("Projection: {:?}", projection_matrix);

        let mvp_matrix =
            projection_matrix
            .dot(&view_matrix)
            .dot(&model_matrix);

        let graphics_matrix = array![
            [Self::COORDS_SCALE, 0.0, 0.0, (Self::WIDTH as f64) / 2.0],
            [0.0, -Self::COORDS_SCALE, 0.0, (Self::HEIGHT as f64) / 2.0],
            [0.0, 0.0, Self::COORDS_SCALE, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
//        let graphics_matrix = array![
//            [1.0, 0.0, 0.0, 0.0],
//            [0.0, 1.0, 0.0, 0.0],
//            [0.0, 0.0, 1.0, 0.0],
//            [0.0, 0.0, 0.0, 1.0],
//        ];

        let red = [1.0, 0.0, 0.0, 1.0];
        let width = 1.0;
        let edges = polytope.get_edges();
        for (point1, point2) in edges {
            let point1_arr = array![point1.0, point1.1, point1.2, 1.0];
            let point2_arr = array![point2.0, point2.1, point2.2, 1.0];
            println!("Point1 raw: {:?}", point1_arr);
            println!("Point2 raw: {:?}", point2_arr);

            let mut point1_projection = mvp_matrix.dot(&point1_arr);
            let mut point2_projection = mvp_matrix.dot(&point2_arr);
            point1_projection /= point1_projection[3];
            point2_projection /= point2_projection[3];
            println!("Point1 pre graphics: {:?}", point1_projection);
            println!("Point2 pre graphics: {:?}", point2_projection);

            let point1_graphical = graphics_matrix.dot(&Self::clip(point1_projection));
            let point2_graphical = graphics_matrix.dot(&Self::clip(point2_projection));
            println!("Point1: {:?}", point1_graphical);
            println!("Point2: {:?}", point2_graphical);

            let coords = [
                point1_graphical[0], point1_graphical[1],
                point2_graphical[0], point2_graphical[1],
            ];
            pw::line(red, width, coords, context.transform, graphics);
        }
    }

    fn clip(arr: Array1<f64>) -> Array1<f64> {
        arr.map(|&x| {
            if x > 1.0 { 1.0 }
            else if x < -1.0 { -1.0 }
            else { x }
        })
    }
}


struct DemoController {
    model: DemoModel,
    view: DemoView,
}

impl DemoController {
    pub fn new() -> DemoController {
        let model = DemoModel::new();
        let view = DemoView::new();
        DemoController { model, view }
    }

    pub fn run(&mut self) {
        info!("Starting demo");

        while let Some(e) = self.view.next_event() {
            trace!("Received event: {:?}", e);

            // handle input events
            e.button(|button_args| self.view.handle_button_event(&button_args));
            e.focus(|focused| self.view.handle_focus_event(focused));

            // update model if needed
            e.update(|update_args| self.model.update(update_args.dt, self.view.input_state()));

            // rerender if needed
            self.view.handle_render_event(&e, &self.model);
        }
    }
}


pub fn run() -> i32 {
    env_logger::init();

    let mut demo_controller = DemoController::new();
    demo_controller.run();

    0
}
