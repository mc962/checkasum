use orbtk::prelude::*;
use crate::gui::input::view;

widget!(MainView{});



impl Template for MainView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        let input_view = view::InputView::create()
            .build(ctx);
        self.name("MainView")
            .child(input_view)
    }
}
