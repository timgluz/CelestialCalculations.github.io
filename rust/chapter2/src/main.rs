extern crate orbtk;

use orbtk::prelude::*;
use std::cell::Cell;

#[derive(Debug, Copy, Clone)]
enum Action {
    ClearText,
    Convert,
    InputChanged(entity),
}

#[derive(Default)]
pub struct MainViewState {
    action: Cell<Option<Action>>,
}

impl MainViewState {
    fn action(&self, action: Action) {
        self.action.set(Some(action));
    }
}

impl State for MainViewState {
    fn update(&self, ctx: &mut Context<'_>) {
        if let Some(action) = self.action.get() {
            match action {
                Action::ClearText => {
                    ctx.widget().set("text_input", String16::from(""));
                    ctx.widget().set("text_result", String16::from(""));
                }
                Action::Convert => {
                    let input = *ctx.widget().get::<f64>("input");
                    let result_txt = format!("{:.9}", input);
                    ctx.widget().set("text_result", String16::from(result_txt));
                }
                Action::InputChanged(entity) => {
                    let value = *ctx.get_widget(entity).get::<f64>("text");
                    *ctx.widget().get_mut::<f64>("input") = value;
                    println!("entry changed: {}", value);
                }
                _ => {
                    println!("unsupported action");
                }
            }

            self.action.set(None);
        }
    }
}

fn create_header(ctx: &mut BuildContext, text: &str) -> Entity {
    TextBlock::create()
        .text(text)
        .selector(Selector::new().with("text-block").class("h1"))
        .build(ctx)
}

widget!(
    MainView<MainViewState> {
        input: f64,
        text_input: String16,
        text_result: String16
    }
);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let clear_state = self.clone_state();
        let convert_state = self.clone_state();
        let change_state = self.clone_state();

        self.name("Chapter.2 - Unit conversion")
            .input(0.0)
            .text_input("0.0")
            .text_result("Result: 0")
            .child(
                Grid::create()
                    .margin(10.0)
                    .columns(Columns::create().column(150.0).column(150.0).build())
                    .child(
                        Stack::create()
                            .attach(Grid::column(0))
                            .child(create_header(ctx, "From unit"))
                            .child(
                                TextBox::create()
                                    .water_mark("From value")
                                    .text(("text_input", id))
                                    .on_changed(move |_, entity| {
                                        change_state.action(Action::InputChanged(entity));
                                    })
                                    .build(ctx),
                            )
                            .child(
                                Button::create()
                                    .text("Clear")
                                    .on_click(move |_| {
                                        clear_state.action(Action::ClearText);
                                        true
                                    })
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .child(
                        Stack::create()
                            .attach(Grid::column(1))
                            .child(create_header(ctx, "To unit"))
                            .child(
                                TextBox::create()
                                    .water_mark("Result")
                                    .text(("text_result", id))
                                    .build(ctx),
                            )
                            .child(
                                Button::create()
                                    .text("Convert")
                                    .on_click(move |_| {
                                        convert_state.action(Action::Convert);
                                        true
                                    })
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}

fn main() {
    Application::new()
        .window(|ctx| {
            Window::create()
                .title("Chapter.2 - unit conversion")
                .position((100.0, 100.0))
                .size(450.0, 450.0)
                .child(MainView::create().build(ctx))
                .build(ctx)
        })
        .run()
}
