// use druid::widget::Container;
use druid::{
    widget::{Flex, Label, SizedBox},
    AppLauncher, Color, Widget, WidgetExt, WindowDesc,
};
use druid_container::{BorderStyle, Container};
fn main() {
    let window = WindowDesc::new(app)
        .title("Border")
        .window_size((600., 600.));

    AppLauncher::with_window(window)
        // .use_simple_logger()
        .launch(())
        .unwrap();
}

fn app() -> impl Widget<()> {
    let button = Label::new("Hello");
    // let container = Container::new(button)
    //     .border(Color::WHITE, 1.0)
    //     .border_left(Some(BorderStyle {
    //         width: 1.0.into(),
    //         color: Color::GREEN.into(),
    //     }));
    // let container = Container::new(button)
    //     .border_left(Some(BorderStyle {
    //         width: 1.0.into(),
    //         color: Color::GREEN.into(),
    //     }))
    //     .border_right(Some(BorderStyle {
    //         width: 1.0.into(),
    //         color: Color::RED.into(),
    //     }));
    let container = Container::new(button)
        .border_left(Some((10.0, Color::GREEN)))
        .border_right(Some((10.0, Color::RED)))
        .border_top(Some((10.0, Color::BLUE)))
        .border_bottom(Some((10.0, Color::YELLOW)));
    SizedBox::new(container).width(300.).height(300.).center()
}
