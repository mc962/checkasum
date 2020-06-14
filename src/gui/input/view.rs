use orbtk::prelude::*;

widget!(InputView{});

impl Template for InputView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("InputView")
            .child(
                Grid::create()
                    .rows(
                        Rows::create()
                            .row(300.0)
                            .row(300.0)
                            .build()
                    ).child(
                    Container::create()
                        .attach(Grid::row(0))
                        .child(
                            TextBlock::create().text("File Path").build(ctx)
                        )
                        .child(
                            TextBox::create().water_mark("Enter file path...").build(ctx)
                        )
                        .build(ctx)
                ).child(
                    Container::create()
                        .attach(Grid::row(1))
                        .child(
                            TextBlock::create().text("Checksum").build(ctx)
                        )
                        .child(
                            TextBox::create().water_mark("Enter checksum...").build(ctx)
                        )
                        .build(ctx)
                ).build(ctx)
            )
    }
}