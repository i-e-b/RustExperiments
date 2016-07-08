#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate piston_window;

use conrod::{Theme, Widget};
use piston_window::{EventLoop, OpenGL, PistonWindow, UpdateEvent, WindowSettings};

type Backend = (piston_window::G2dTexture<'static>, piston_window::Glyphs);
type Ui = conrod::Ui<Backend>;
type UiCell<'a> = conrod::UiCell<'a, Backend>;

fn main() {
    let opengl = OpenGL::V3_2;

    // Create a window
    let mut window: PistonWindow =
        WindowSettings::new("Text Demo", [1080, 720])
            .opengl(opengl).exit_on_esc(true).build().expect("Window error");

    // create the UI to go in the window
    let mut ui = {
        let assets = find_folder::Search::KidsThenParents(3, 5)
            .for_folder("assets").expect("asset error");
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        let theme = Theme::default();
        let glyph_cache = piston_window::Glyphs::new(&font_path, window.factory.clone()).expect("glyph cache error");
        Ui::new(glyph_cache, theme)
    };

    // set the window frame rate
    window.set_ups(30);

    // Handle window events
    while let Some(event) = window.next() {
        ui.handle_event(event.clone());
        event.update(|_| ui.set_widgets(set_ui));
        window.draw_2d(&event, |c,g| ui.draw_if_changed(c,g));
    }
}

fn set_ui(ref mut ui: UiCell) {
    use conrod::{Canvas, color, Colorable, Positionable, Scalar, Sizeable, Text};

    widget_ids!{ // make a new const widget id for each widget
        MASTER, LEFT_COL, MIDDLE_COL, RIGHT_COL, LEFT_TEXT,
        MIDDLE_TEXT, RIGHT_TEXT,
    }

    Canvas::new().flow_right(&[
                             (LEFT_COL, Canvas::new().color(color::BLACK)),
                             (MIDDLE_COL, Canvas::new().color(color::DARK_CHARCOAL)),
                             (RIGHT_COL, Canvas::new().color(color::CHARCOAL)),
                             ]).set(MASTER, ui);

    const DEMO_TEXT: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
        Mauris aliquet porttitor tellus vel euismod. Integer lobortis volutpat bibendum. Nulla \
        finibus odio nec elit condimentum, rhoncus fermentum purus lacinia. Interdum et malesuada \
        fames ac ante ipsum primis in faucibus. Cras rhoncus nisi nec dolor bibendum pellentesque. \
        Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. \
        Quisque commodo nibh hendrerit nunc sollicitudin sodales. Cras vitae tempus ipsum. Nam \
        magna est, efficitur suscipit dolor eu, consectetur consectetur urna.";

    const PAD: Scalar = 20.0;

    Text::new(DEMO_TEXT)
        .color(color::LIGHT_RED)
        .padded_w_of(LEFT_COL, PAD)
        .mid_top_with_margin_on(LEFT_COL, PAD)
        .align_text_left()
        .line_spacing(10.0)
        .set(LEFT_TEXT, ui);

    Text::new(DEMO_TEXT)
        .color(color::LIGHT_GREEN)
        .padded_w_of(MIDDLE_COL, PAD)
        .middle_of(MIDDLE_COL)
        .align_text_middle()
        .line_spacing(2.5)
        .set(MIDDLE_TEXT, ui);

    Text::new(DEMO_TEXT)
        .color(color::LIGHT_BLUE)
        .padded_w_of(RIGHT_COL, PAD)
        .mid_bottom_with_margin_on(RIGHT_COL, PAD)
        .align_text_right()
        .line_spacing(5.0)
        .set(RIGHT_TEXT, ui);
}


