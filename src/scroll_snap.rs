use druid::widget::Scroll;
use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size,
    UpdateCtx, Vec2, Widget,
};

pub struct ScrollSnap<T, W> {
    scroll: Scroll<T, W>,
    snap_horizontal: Box<dyn Fn(&T, &Env) -> bool>,
    snap_vertical: Box<dyn Fn(&T, &Env) -> bool>,
    snap: bool,
}

impl<T, W: Widget<T>> ScrollSnap<T, W> {
    pub fn new(scroll: Scroll<T, W>) -> Self {
        Self {
            scroll,
            snap_horizontal: Box::new(|_, _| false),
            snap_vertical: Box::new(|_, _| false),
            snap: false,
        }
    }

    pub fn with_snap_horizontal(mut self, snap: impl Fn(&T, &Env) -> bool + 'static) -> Self {
        self.snap_horizontal = Box::new(snap);
        self
    }

    pub fn with_snap_vertical(mut self, snap: impl Fn(&T, &Env) -> bool + 'static) -> Self {
        self.snap_vertical = Box::new(snap);
        self
    }
}

impl<T: Data, W: Widget<T>> Widget<T> for ScrollSnap<T, W> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.scroll.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.scroll.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.scroll.update(ctx, old_data, data, env);

        ctx.request_layout();

        if (self.snap_horizontal)(data, env) && self.snap {
            self.scroll.scroll_by(Vec2::new(f64::MAX, 0.0));
        }
        if (self.snap_vertical)(data, env) && self.snap {
            self.scroll.scroll_by(Vec2::new(0.0, f64::MAX));
        }
        self.snap = false;
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let size = self.scroll.layout(ctx, bc, data, env);
        self.snap = true;
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.scroll.paint(ctx, data, env);
    }
}
