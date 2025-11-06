use neon::prelude::*;
extern crate glob;
use src::settings::read_json_for_settings;
mod src;
use crate::src::game_chooser;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("emerald_rom", create_emerald_rom)?;
    Ok(())
}

fn create_emerald_rom(mut cx: FunctionContext) -> JsResult<JsNumber>{
    let json_string = cx.argument::<JsString>(0)?;
    let stringy = json_string.value(&mut cx);
    let mut settings = read_json_for_settings(stringy).unwrap();

    game_chooser::main_randomizer_script(&mut settings);
    Ok(cx.number(200))
}