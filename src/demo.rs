use env_logger;
use piston_window as pw;


struct DemoModel {
    pub x1: f64, pub y1: f64,
    pub x2: f64, pub y2: f64,
}

impl DemoModel {
    pub fn new() -> DemoModel {
        DemoModel { x1: 20.0, y1: 20.0, x2: 400.0, y2: 400.0 }
    }
}


struct DemoView {
    window: pw::PistonWindow,
}

impl DemoView {
    pub fn new() -> DemoView {
        let window =
            pw::WindowSettings::new("threedy", [640, 480])
            .exit_on_esc(true)
            .build()
            .expect("window init failed");

        DemoView { window }
    }

    pub fn next_event(&mut self) -> Option<pw::Event> {
        self.window.next()
    }

    pub fn handle_render_event(&mut self, event: &pw::Event, model: &DemoModel) {
        self.window.draw_2d(event, |context, graphics| {
            Self::render(context, graphics, model);
        });
    }

    fn render(context: pw::Context, graphics: &mut pw::G2d, model: &DemoModel) {
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
            debug!("Received event: {:?}", e);
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
