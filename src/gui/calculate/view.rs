use orbtk::prelude::*;

widget!(CalculateView{});

impl Template for CalculateView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("CalculateView")
            .child(
                Grid::create()
                    .rows(
                        Rows::create()
                            .row(300.0)
                            .row(300.0)
                            .row(300.0)
                            .build()
                    ).child(
                    Container::create()
                        .attach(Grid::row(2))
                        .child(
                            Button::create()
                                .text("Check File")
                                .build(ctx)
                        )
                        .build(ctx)
                ).build(ctx)
            )
    }
}