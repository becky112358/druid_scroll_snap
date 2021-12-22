use std::sync::Arc;

use druid::widget::{Button, Controller, Flex, Label, List, RadioGroup, Scroll};
use druid::{
    theme, AppLauncher, Color, Data, Env, FontDescriptor, FontWeight, Lens, UpdateCtx, Vec2,
    Widget, WidgetExt, WindowDesc,
};

const WINDOW_SIZE: f64 = 200.0;
const WIDGET_SPACING: f64 = 20.0;
const BIG_NUMBER: f64 = f64::MAX;   // todo...
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
struct ScrollSnap {
    indices: Arc<Vec<usize>>,
    next_index: usize,
    snap_user_requested: bool,
    snap_required: bool,
}

impl ScrollSnap {
    fn new() -> Self {
        Self {
            indices: Arc::new(Vec::new()),
            next_index: 0,
            snap_user_requested: true,
            snap_required: false,
        }
    }
}

struct ScrollExternal;

impl<W: Widget<ScrollSnap>> Controller<ScrollSnap, Scroll<ScrollSnap, W>> for ScrollExternal {
    fn update(
        &mut self,
        child: &mut Scroll<ScrollSnap, W>,
        ctx: &mut UpdateCtx,
        old_data: &ScrollSnap,
        data: &ScrollSnap,
        env: &Env,
    ) {
        if data.snap_user_requested && data.snap_required {
            child.scroll_by(Vec2::new(0.0, BIG_NUMBER));
        }

        child.update(ctx, old_data, data, env);
    }
}

pub fn main() {
    let window = WindowDesc::new(build_window)
        .title("Scroll Snap")
        .window_size((WINDOW_SIZE, WINDOW_SIZE));

    let initial_state = ScrollSnap::new();

    AppLauncher::with_window(window)
        .configure_env(|env, _| env.set(theme::UI_FONT, FontDescriptor::default()))
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_window() -> impl Widget<ScrollSnap> {
    Flex::column()
        .with_child(build_button_add())
        .with_flex_child(build_list(), 1.0)
        .with_child(build_buttons_snap_subtract())
}

fn build_button_add() -> impl Widget<ScrollSnap> {
    Button::new("Click me!").on_click(|_, data: &mut ScrollSnap, _| {
        Arc::make_mut(&mut data.indices).push(data.next_index);
        if data.next_index == WORDS.len() - 1 {
            data.next_index = 0
        } else {
            data.next_index += 1;
        }

        data.snap_required = data.snap_user_requested;
    })
}

fn build_list() -> impl Widget<ScrollSnap> {
    Flex::column().with_flex_child(
        Scroll::new(List::new(build_individual_item).lens(ScrollSnap::indices))
            .vertical()
            .controller(ScrollExternal {}),
        1.0,
    )
}

fn build_individual_item() -> impl Widget<usize> {
    Label::new(|data: &usize, _env: &Env| WORDS[*data].to_string()).fix_width(60.0)
}

fn build_buttons_snap_subtract() -> impl Widget<ScrollSnap> {
    let pause = RadioGroup::new(vec![("||", false)]).lens(ScrollSnap::snap_user_requested);

    let snap = RadioGroup::new(vec![("▼", true)]).lens(ScrollSnap::snap_user_requested);

    let clear_x = Label::new("X")
        .with_font(FontDescriptor::default().with_weight(FontWeight::BOLD))
        .with_text_color(COLOR_CLEAR);

    let clear_button = Button::from_label(clear_x).on_click(move |_, data: &mut ScrollSnap, _| {
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
