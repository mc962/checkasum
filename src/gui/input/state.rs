use orbtk::prelude::*;

#[derive(Default, AsAny)]
pub struct InputViewState {
    action: Option<Action>,
    file_path: Entity,
    checksum: Entity
}

impl InputViewState {}

impl State for InputViewState {
    fn init(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        self.file_path = ctx
            .entity_of_child("file_path_text_box")
            .expect("InputViewState.init: File Path text box could not be found.");
        self.checksum = ctx
            .entity_of_child("checksum_text_box")
            .expect("InputViewState.init: Checksum text box could not be found.");
    }

    fn update(&mut self, registry: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.action {
            match action {
                Action::FilePathTextChanged(entity) => {
                    let input_val = ctx.get_widget(entity).get("value")
                },
                Action::ChecksumTextChanged(entity) => {

                }
            }
        }

        self.action = None;
    }
}

pub enum Action {
    FilePathTextChanged,
    ChecksumTextChanged
}