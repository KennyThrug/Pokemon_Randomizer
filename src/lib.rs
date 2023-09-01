use neon::prelude::*;
mod src;
use crate::src::gen3::emerald::startup;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("get", create_gen_3_rom)?;
    Ok(())
}

fn create_gen_3_rom(mut cx: FunctionContext) -> JsResult<JsNumber>{
    println!("Hello");
    startup::randomize_pokemon();
    Ok(cx.number(200))
}