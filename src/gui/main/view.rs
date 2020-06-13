use orbtk::prelude::*;

widget!(MainView{});

impl Template for MainView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self{
        self.name("MainView")
            .child(TextBlock::create().text("Checkasum").build(ctx))
    }
}
