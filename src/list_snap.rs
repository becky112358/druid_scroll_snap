use druid::widget::{Controller, List, ListIter, Scroll};
use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    Selector, Size, UpdateCtx, Vec2, Widget,
};

use crate::types::AppData;

const LIST_EXTENDED_AT_END: Selector<()> = Selector::new("list has been extended at the end");

pub struct ListSnap<T> {
    list: List<T>,
    size: Size,
}

impl<T> ListSnap<T> {
    pub fn new(list: List<T>) -> Self {
        Self {
            list,
            size: Size::ZERO,
        }
    }
}

pub struct ScrollSnap;

impl<W: Widget<AppData>> Controller<AppData, Scroll<AppData, W>> for ScrollSnap {
    fn event(
        &mut self,
        child: &mut Scroll<AppData, W>,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppData,
        env: &Env,
    ) {
        child.event(ctx, event, data, env);

        match event {
            Event::Command(c) => {
                if c.get(LIST_EXTENDED_AT_END).is_some() {
                    if data.snap_user_requested {
                        child.scroll_by(Vec2::new(0.0, f64::MAX));
                    }
                    ctx.set_handled();
                }
            }
            _ => (),
        }
    }
}

impl<C: Data, T: ListIter<C>> Widget<T> for ListSnap<C> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.list.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.list.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.list.update(ctx, old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let new_size = self.list.layout(ctx, bc, data, env);
        if new_size.height > self.size.height {
            let _ = ctx.submit_command(LIST_EXTENDED_AT_END);
        }
        self.size = new_size;
        new_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.list.paint(ctx, data, env);
    }
}
