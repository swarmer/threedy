use env_logger;
use piston_window as pw;
use piston_window::{ButtonEvent, FocusEvent, UpdateEvent};

use input_state::InputState;


#[derive(Clone, Debug, PartialEq)]
struct DemoModel {
    pub x1: f64, pub y1: f64,
    pub x2: f64, pub y2: f64,
}

impl DemoModel {
    pub fn new() -> DemoModel {
        DemoModel { x1: 20.0, y1: 20.0, x2: 400.0, y2: 400.0 }
    }

    pub fn update(&mut self, dt: f64, input_state: InputState) {
        trace!("Updating model");

        self.y1 -= 100.0 * dt * match input_state.up { pw::ButtonState::Press => 1.0, _ => 0.0 };
        self.y1 += 100.0 * dt * match input_state.down { pw::ButtonState::Press => 1.0, _ => 0.0 };
        self.x1 -= 100.0 * dt * match input_state.left { pw::ButtonState::Press => 1.0, _ => 0.0 };
        self.x1 += 100.0 * dt * match input_state.right { pw::ButtonState::Press => 1.0, _ => 0.0 };
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

        let red = [1.0, 0.0, 0.0, 1.0];
        let width = 1.0;
        let coords = [model.x1, model.y1, model.x2, model.y2];
        pw::line(red, width, coords, context.transform, graphics);
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
