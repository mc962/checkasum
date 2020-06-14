use orbtk::prelude::*;

widget!(OutputView{});

impl Template for OutputView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("OutputView")
            .child(
                Grid::create()
                    .rows(
                        Rows::create()
                            .row(300.0)
                            .build()
                    ).child(
                    Container::create()
                        .attach(Grid::row(0))
                        .child(
                            TextBlock::create().text("Status: ").build(ctx)
                        )
                        .build(ctx)
                ).build(ctx)
            )
    }
}