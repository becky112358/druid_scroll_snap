use std::sync::Arc;

use druid::widget::{Button, Controller, Flex, Label, List, RadioGroup, Scroll};
use druid::{
    theme, AppDelegate, AppLauncher, Color, Command, Data, DelegateCtx, Env, FontDescriptor,
    FontWeight, Handled, Lens, Selector, Size, Target, UpdateCtx, Vec2, Widget, WidgetExt,
    WindowDesc,
};

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

const COMMAND_SIZE: Selector<Size> = Selector::new("size");

#[derive(Clone, Data, Lens)]
struct ScrollSnap {
    indices: Arc<Vec<usize>>,
    next_index: usize,
    snap_user_requested: bool,
    list_size: Size,
}

impl ScrollSnap {
    fn new() -> Self {
        Self {
            indices: Arc::new(Vec::new()),
            next_index: 0,
            snap_user_requested: true,
            list_size: Size::ZERO,
        }
    }
}

struct ScrollInternal;
struct ScrollExternal;

impl<W: Widget<ScrollSnap>> Controller<ScrollSnap, W> for ScrollInternal {
    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut UpdateCtx,
        old_data: &ScrollSnap,
        data: &ScrollSnap,
        env: &Env,
    ) {
        let new_size = ctx.size();
        if new_size != data.list_size {
            let _ = ctx
                .get_external_handle()
                .submit_command(COMMAND_SIZE, new_size, Target::Auto);
        }

        child.update(ctx, old_data, data, env);
    }
}

impl<W: Widget<ScrollSnap>> Controller<ScrollSnap, Scroll<ScrollSnap, W>> for ScrollExternal {
    fn update(
        &mut self,
        child: &mut Scroll<ScrollSnap, W>,
        ctx: &mut UpdateCtx,
        old_data: &ScrollSnap,
        data: &ScrollSnap,
        env: &Env,
    ) {
        if data.snap_user_requested && data.list_size.height > old_data.list_size.height {
            child.scroll_by(Vec2::new(
                0.0,
                data.list_size.height - old_data.list_size.height,
            ));
        }

        child.update(ctx, old_data, data, env);
    }
}

struct Delegate;

impl AppDelegate<ScrollSnap> for Delegate {
    fn command(
        &mut self,
        _: &mut DelegateCtx,
        _: Target,
        cmd: &Command,
        data: &mut ScrollSnap,
        _: &Env,
    ) -> Handled {
        if let Some(size) = cmd.get(COMMAND_SIZE) {
            data.list_size = *size;
            Handled::Yes
        } else {
            Handled::No
        }
    }
}

pub fn main() {
    let window = WindowDesc::new(build_window)
        .title("Scroll Snap")
        .window_size((WINDOW_SIZE, WINDOW_SIZE));

    let initial_state = ScrollSnap::new();

    AppLauncher::with_window(window)
        .configure_env(|env, _| env.set(theme::UI_FONT, FontDescriptor::default()))
        .delegate(Delegate {})
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
    })
}

fn build_list() -> impl Widget<ScrollSnap> {
    Flex::column().with_flex_child(
        Scroll::new(
            List::new(build_individual_item)
                .lens(ScrollSnap::indices)
                .controller(ScrollInternal {}),
        )
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
