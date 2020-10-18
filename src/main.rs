mod controller;
mod model;
mod view;

fn main() {
  let ui_result = view::UI::new();
  if ui_result.is_err() {
    println!("UI init failed: {}", ui_result.unwrap_err());
    return;
  }

  // tests done, let's go
  let mut board = model::Board::new(); // model
  let ui = ui_result.unwrap(); // view
  let game = controller::Game::new(); // controller

  loop {
    let user_input = ui.wait_for_user_input();
    match user_input {
      view::UserInput::UserWantsToQuit => break,
      input => game.handle_input(&input, &mut board),
    }

    game.step(&mut board);
    ui.draw(&board);
  }

  ui.destroy();
}
