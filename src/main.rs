use std::sync::Arc;

use druid::widget::{Button, Flex, Label, List, RadioGroup, Scroll};
use druid::{
    theme, AppLauncher, Color, Data, Env, FontDescriptor, FontWeight, Lens, Widget, WidgetExt,
    WindowDesc,
};

mod scroll_snap;
use crate::scroll_snap::ScrollSnap;

const WINDOW_SIZE: f64 = 200.0;
const WIDGET_SPACING: f64 = 20.0;
const COLOR_CLEAR: Color = Color::rgb8(0xff, 0x30, 0x30);

const WORDS: &[&str] = &[
    "We",
    "do",
    "not",
    "inherit",
    "the",
    "earth",
    "from",
    "our",
    "ancestors,",
    "we",
    "borrow",
    "it",
    "from",
    "our",
    "children.",
];

#[derive(Clone, Data, Lens)]
struct AppData {
    indices: Arc<Vec<usize>>,
    next_index: usize,
    snap_user_requested: bool,
}

impl AppData {
    fn new() -> Self {
        Self {
            indices: Arc::new(Vec::new()),
            next_index: 0,
            snap_user_requested: true,
        }
    }
}

pub fn main() {
    let window = WindowDesc::new(build_window)
        .title("Scroll Snap")
        .window_size((WINDOW_SIZE, WINDOW_SIZE));

    let initial_state = AppData::new();

    AppLauncher::with_window(window)
        .configure_env(|env, _| env.set(theme::UI_FONT, FontDescriptor::default()))
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_window() -> impl Widget<AppData> {
    Flex::column()
        .with_child(build_button_add())
        .with_flex_child(build_list(), 1.0)
        .with_child(build_buttons_snap_subtract())
}

fn build_button_add() -> impl Widget<AppData> {
    Button::new("Click me!").on_click(|_, data: &mut AppData, _| {
        Arc::make_mut(&mut data.indices).push(data.next_index);
        if data.next_index == WORDS.len() - 1 {
            data.next_index = 0
        } else {
            data.next_index += 1;
        }
    })
}

fn build_list() -> impl Widget<AppData> {
    Flex::column().with_flex_child(
        ScrollSnap::new(
            Scroll::new(List::new(build_individual_item).lens(AppData::indices)).vertical(),
        )
        .with_snap_vertical(|data: &AppData, _| data.snap_user_requested),
        1.0,
    )
}

fn build_individual_item() -> impl Widget<usize> {
    Label::new(|data: &usize, _env: &Env| WORDS[*data].to_string()).fix_width(60.0)
}

fn build_buttons_snap_subtract() -> impl Widget<AppData> {
    let pause = RadioGroup::new(vec![("||", false)]).lens(AppData::snap_user_requested);

    let snap = RadioGroup::new(vec![("▼", true)]).lens(AppData::snap_user_requested);

    let clear_x = Label::new("X")
        .with_font(FontDescriptor::default().with_weight(FontWeight::BOLD))
        .with_text_color(COLOR_CLEAR);

    let clear_button = Button::from_label(clear_x).on_click(move |_, data: &mut AppData, _| {
        data.next_index = 0;
        data.indices = Arc::new(Vec::new())
    });

    Flex::row()
        .with_child(pause)
        .with_spacer(WIDGET_SPACING)
        .with_child(snap)
        .with_spacer(WIDGET_SPACING)
        .with_child(clear_button)
}
