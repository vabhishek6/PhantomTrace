use vergen_gix::{BuildBuilder, Emitter, GixBuilder};

fn main() -> anyhow::Result<()> {
    Emitter::default()
        .add_instructions(&BuildBuilder::all_build()?)?
        .add_instructions(&GixBuilder::all_git()?)?
        .emit()
}
