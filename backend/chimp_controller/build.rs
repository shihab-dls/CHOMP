use targeting::root_schema_builder;

fn main() {
    let schema = root_schema_builder().finish();
    cynic_codegen::register_schema("targeting")
        .from_sdl(&schema.sdl())
        .unwrap();
}
