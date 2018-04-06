use env_logger;
use piston_window as pw;


struct Demo {
    window: pw::PistonWindow,
}

impl Demo {
    pub fn new() -> Demo {
        let window =
            pw::WindowSettings::new("threedy", [640, 480])
            .exit_on_esc(true)
            .build()
            .expect("window init failed");

        Demo { window }
    }

    pub fn run(&mut self) {
        info!("Starting demo");

        while let Some(e) = self.window.next() {
            debug!("Received event: {:?}", e);

            self.window.draw_2d(&e, |c, g| {
                pw::clear([1.0; 4], g);

                pw::line([1.0, 0.0, 0.0, 1.0], 1.0, [20.0, 20.0, 400.0, 400.0], c.transform, g);
            });
        }
    }
}


pub fn run() -> i32 {
    env_logger::init();

    let mut demo = Demo::new();
    demo.run();

    0
}
