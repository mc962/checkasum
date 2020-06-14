use orbtk::prelude::*;
use crate::gui::input;
use crate::gui::calculate;
use crate::gui::output;

widget!(MainView{});

impl Template for MainView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        let input_view = input::view::InputView::create()
            .build(ctx);
        let calculate_view = calculate::view::CalculateView::create()
            .build(ctx);
        let output_view = output::view::OutputView::create()
            .build(ctx);

        self.name("MainView")
            .child(
                Grid::create()
                    .columns(
                        Columns::create()
                            .column(200.0)
                            .column(200.0)
                            .column(200.0)
                            .build()
                    ).child(
                        Container::create()
                            .attach(Grid::column(0))
                            .child(
                                input_view
                            ).build(ctx)
                    ).child(
                        Container::create()
                            .attach(Grid::column(1))
                            .child(
                                calculate_view
                            ).build(ctx)
                    ).child(
                        Container::create()
                            .attach(Grid::column(2))
                            .child(
                                output_view
                            ).build(ctx)
                    ).build(ctx)
            )
    }
}
