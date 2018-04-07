use env_logger;
use ndarray::{Array};
use piston_window as pw;
use piston_window::{ButtonEvent, FocusEvent, UpdateEvent};

use input_state::InputState;
use polytopes::{Point3d, Edge, Polytope, Cube};


#[derive(Debug)]
struct DemoModel {
    polytope: Box<Cube>,
}

impl DemoModel {
    pub fn new() -> DemoModel {
        let center = (3.0, 5.0, 2.0);
        let height = 1.0;
        let polytope = Box::new(Cube { center, height });
        DemoModel { polytope }
    }

    pub fn update(&mut self, dt: f64, input_state: InputState) {
        trace!("Updating model");

        self.polytope.center.2 -= dt * match input_state.up { pw::ButtonState::Press => 1.0, _ => 0.0 };
        self.polytope.center.2 += dt * match input_state.down { pw::ButtonState::Press => 1.0, _ => 0.0 };
        self.polytope.center.0 -= dt * match input_state.left { pw::ButtonState::Press => 1.0, _ => 0.0 };
        self.polytope.center.0 += dt * match input_state.right { pw::ButtonState::Press => 1.0, _ => 0.0 };
    }
}


struct DemoView {
    input_state: InputState,
    window: pw::PistonWindow,
}

impl DemoView {
    pub fn new() -> DemoView {
        let input_state = InputState::new();

        let window =
            pw::WindowSettings::new("threedy", [640, 480])
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
                self.input_state.up = button_args.state;
            },
            pw::Button::Keyboard(pw::Key::A) => {
                self.input_state.left = button_args.state;
            },
            pw::Button::Keyboard(pw::Key::S) => {
                self.input_state.down = button_args.state;
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
        let transformation_matrix = array![
            [1.0, 0.0, -0.3, 0.0],
            [0.0, 1.0, -0.3, 0.0],
        ] * 50.0;

        let red = [1.0, 0.0, 0.0, 1.0];
        let width = 1.0;
        let edges = polytope.edges();
        for (point1, point2) in edges {
            let point1_arr = array![point1.0, point1.1, point1.2, 1.0];
            let point2_arr = array![point2.0, point2.1, point2.2, 1.0];

            let point1_transformed = transformation_matrix.dot(&point1_arr);
            let point2_transformed = transformation_matrix.dot(&point2_arr);

            let coords = [
                point1_transformed[0], point1_transformed[1],
                point2_transformed[0], point2_transformed[1],
            ];
            pw::line(red, width, coords, context.transform, graphics);
        }
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
