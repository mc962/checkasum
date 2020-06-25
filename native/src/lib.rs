use neon::prelude::*;

pub mod error;
pub mod hashing;

use hashing::check_file;

#[no_mangle]
pub fn gui_check_file(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let method = cx.argument::<JsString>(0).unwrap().value();
    let path = cx.argument::<JsString>(1).unwrap().value();
    let expected = cx.argument::<JsString>(2).unwrap().value();

    let result = check_file(&method, &path, &expected).unwrap_or(false);
    Ok(cx.boolean(result))
}

register_module!(mut m, {
    m.export_function("checkFile", gui_check_file)
});