use crate::game::Game;

mod game;
mod graphics;
mod mesh;

fn main() -> anyhow::Result<()> {
    Game::new()?.run()?;
    Ok(())
}
