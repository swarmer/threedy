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

    pub fn handle_render(&mut self, event: &pw::Event, model: &DemoModel) {
        self.window.draw_2d(event, |ctx, gr| {
            pw::clear([1.0; 4], gr);
            pw::line([1.0, 0.0, 0.0, 1.0], 1.0, [20.0, 20.0, 400.0, 400.0], ctx.transform, gr);
        });
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
            self.view.handle_render(&e, &self.model);
        }
    }
}


pub fn run() -> i32 {
    env_logger::init();

    let mut demo_controller = DemoController::new();
    demo_controller.run();

    0
}
