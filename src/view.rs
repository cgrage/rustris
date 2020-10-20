extern crate pancurses;

use model::{Board, CellVal, UiState};

pub enum UserInput {
    UserWantsToQuit,
    MoveLeft,
    MoveRight,
    MoveDown,
    DropDown,
    RotateLeft,
    RotateRight,
    ChangeUI,
    NoInput,
}

#[derive(Debug)]
pub struct UI {
    screen: pancurses::Window,
    app_win: pancurses::Window,
    panel_1: pancurses::Window,
    panel_2: pancurses::Window,
    panel_3: pancurses::Window,
}

impl UI {
    const MIN_WIDTH: i32 = 72;
    const MIN_HEIGHT: i32 = 22;

    pub fn new() -> Result<UI, String> {
        let screen = pancurses::initscr();
        let width = screen.get_max_x();
        let height = screen.get_max_x();

        if width < UI::MIN_WIDTH || height < UI::MIN_HEIGHT {
            pancurses::endwin();
            return Err(format!(
                "Not enough space in terminal; need {}x{}, have {}x{}",
                UI::MIN_WIDTH,
                UI::MIN_HEIGHT,
                width,
                height
            ));
        }
        screen.clear();
        screen.timeout(30);
        screen.keypad(true);
        pancurses::noecho();
        pancurses::cbreak();
        pancurses::curs_set(0);

        let app_win = UI::create_app_win(&screen);
        return Ok(UI {
            screen: screen,
            panel_1: UI::create_panel_1(&app_win),
            panel_2: UI::create_panel_2(&app_win),
            panel_3: UI::create_panel_3(&app_win),
            app_win: app_win,
        });
    }

    fn create_app_win(screen: &pancurses::Window) -> pancurses::Window {
        let app_win = match screen.subwin(UI::MIN_HEIGHT, UI::MIN_WIDTH, 0, 0) {
            Ok(win) => win,
            Err(code) => panic!("pancurses subwin function failed w/ result code {}", code),
        };
        return app_win;
    }

    pub fn destroy(&self) {
        pancurses::endwin();
    }

    pub fn wait_for_user_input(&self) -> UserInput {
        let ch = self.screen.getch();
        match ch {
            Some(pancurses::Input::Character('q')) => UserInput::UserWantsToQuit,
            Some(pancurses::Input::KeyLeft) => UserInput::RotateLeft,
            Some(pancurses::Input::KeyRight) => UserInput::RotateRight,
            Some(pancurses::Input::Character('a')) => UserInput::MoveLeft,
            Some(pancurses::Input::Character('d')) => UserInput::MoveRight,
            Some(pancurses::Input::Character('s')) => UserInput::MoveDown,
            Some(pancurses::Input::Character('w')) => UserInput::DropDown,
            Some(pancurses::Input::Character(' ')) => UserInput::ChangeUI,
            _ => UserInput::NoInput,
        }
    }

    pub fn draw(&self, board: &Board) {
        self.draw_board(&board);

        self.app_win.touch();
        self.app_win.refresh();
    }

    fn draw_board(&self, board: &Board) {
        for y in 0..board.height() {
            for x in 0..board.width() {
                let val = board.cell_value(x, y);
                self.panel_2.mv(1 + y, 2 + 2 * x);
                self.panel_2.printw(self.cell_string(&val));
            }
        }
    }

    fn cell_string(&self, val: &CellVal) -> &str {
        return match val {
            CellVal::Free => "  ",
            CellVal::Garbage => "{}",
            CellVal::ActivePiece => "[]",
            CellVal::OutOfBoard => "??",
        };
    }

    pub fn change(&self, state: &mut UiState) {
        match state.style {
            0 => {
                UI::draw_panel_border(&self.panel_2, 1);
                state.style = 1;
            }
            1 => {
                UI::draw_panel_border(&self.panel_2, 2);
                state.style = 2;
            }
            _ => {
                UI::draw_panel_border(&self.panel_2, 0);
                state.style = 0;
            }
        }
    }

    fn create_panel_1(app_win: &pancurses::Window) -> pancurses::Window {
        let panel = match app_win.subwin(22, 24, 0, 0) {
            Ok(win) => win,
            Err(code) => panic!("pancurses subwin function failed w/ result code {}", code),
        };
        panel.mvaddstr(00, 0, "+                       ");
        panel.mvaddstr(01, 0, " This is RUSTRIS,       ");
        panel.mvaddstr(02, 0, "                        ");
        panel.mvaddstr(03, 0, " a tile matching video  ");
        panel.mvaddstr(04, 0, " game written in the    ");
        panel.mvaddstr(05, 0, " rust programming       ");
        panel.mvaddstr(06, 0, " language.              ");
        panel.mvaddstr(07, 0, "                        ");
        panel.mvaddstr(08, 0, " Please find the        ");
        panel.mvaddstr(09, 0, " source code in         ");
        panel.mvaddstr(10, 0, " github.                ");
        panel.mvaddstr(11, 0, "                        ");
        panel.mvaddstr(12, 0, " Key Mappings:          ");
        panel.mvaddstr(13, 0, "     q - Quit           ");
        panel.mvaddstr(14, 0, "     a - Move left      ");
        panel.mvaddstr(15, 0, "     d - Move right     ");
        panel.mvaddstr(16, 0, "     s - Move down      ");
        panel.mvaddstr(17, 0, "     w - Drop           ");
        panel.mvaddstr(18, 0, "  Left - Rotate         ");
        panel.mvaddstr(19, 0, " Right - Rotate         ");
        panel.mvaddstr(20, 0, " Space - Change UI      ");
        panel.mvaddstr(21, 0, "+                       ");
        return panel;
    }

    fn create_panel_2(app_win: &pancurses::Window) -> pancurses::Window {
        let panel = match app_win.subwin(22, 24, 0, 24) {
            Ok(win) => win,
            Err(code) => panic!("pancurses subwin function failed w/ result code {}", code),
        };
        UI::draw_panel_border(&panel, 0);
        return panel;
    }

    fn draw_panel_border(panel: &pancurses::Window, variant: i32) {
        match variant {
            0 => {
                panel.mvaddstr(00, 0, "-+--------------------+-");
                panel.mvaddstr(01, 0, " :                    : ");
                panel.mvaddstr(02, 0, " :                    : ");
                panel.mvaddstr(03, 0, " :                    : ");
                panel.mvaddstr(04, 0, " :                    : ");
                panel.mvaddstr(05, 0, " :                    : ");
                panel.mvaddstr(06, 0, " :                    : ");
                panel.mvaddstr(07, 0, " :                    : ");
                panel.mvaddstr(08, 0, " :                    : ");
                panel.mvaddstr(09, 0, " :                    : ");
                panel.mvaddstr(10, 0, " :                    : ");
                panel.mvaddstr(11, 0, " :                    : ");
                panel.mvaddstr(12, 0, " :                    : ");
                panel.mvaddstr(13, 0, " :                    : ");
                panel.mvaddstr(14, 0, " :                    : ");
                panel.mvaddstr(15, 0, " :                    : ");
                panel.mvaddstr(16, 0, " :                    : ");
                panel.mvaddstr(17, 0, " :                    : ");
                panel.mvaddstr(18, 0, " :                    : ");
                panel.mvaddstr(19, 0, " :                    : ");
                panel.mvaddstr(20, 0, " :                    : ");
                panel.mvaddstr(21, 0, "-+--------------------+-");
            }
            1 => {
                panel.mvaddstr(00, 0, "########################");
                panel.mvaddstr(01, 0, "##                    ##");
                panel.mvaddstr(02, 0, "##                    ##");
                panel.mvaddstr(03, 0, "##                    ##");
                panel.mvaddstr(04, 0, "##                    ##");
                panel.mvaddstr(05, 0, "##                    ##");
                panel.mvaddstr(06, 0, "##                    ##");
                panel.mvaddstr(07, 0, "##                    ##");
                panel.mvaddstr(08, 0, "##                    ##");
                panel.mvaddstr(09, 0, "##                    ##");
                panel.mvaddstr(10, 0, "##                    ##");
                panel.mvaddstr(11, 0, "##                    ##");
                panel.mvaddstr(12, 0, "##                    ##");
                panel.mvaddstr(13, 0, "##                    ##");
                panel.mvaddstr(14, 0, "##                    ##");
                panel.mvaddstr(15, 0, "##                    ##");
                panel.mvaddstr(16, 0, "##                    ##");
                panel.mvaddstr(17, 0, "##                    ##");
                panel.mvaddstr(18, 0, "##                    ##");
                panel.mvaddstr(19, 0, "##                    ##");
                panel.mvaddstr(20, 0, "##                    ##");
                panel.mvaddstr(21, 0, "########################");
            }
            _ => {
                panel.mvaddstr(00, 0, " .-=-._.-=-._.-=-._.-=. ");
                panel.mvaddstr(01, 0, "(                      )");
                panel.mvaddstr(02, 0, " )                    ( ");
                panel.mvaddstr(03, 0, "(                      )");
                panel.mvaddstr(04, 0, " )                    ( ");
                panel.mvaddstr(05, 0, "(                      )");
                panel.mvaddstr(06, 0, " )                    ( ");
                panel.mvaddstr(07, 0, "(                      )");
                panel.mvaddstr(08, 0, " )                    ( ");
                panel.mvaddstr(09, 0, "(                      )");
                panel.mvaddstr(10, 0, " )                    ( ");
                panel.mvaddstr(11, 0, "(                      )");
                panel.mvaddstr(12, 0, " )                    ( ");
                panel.mvaddstr(13, 0, "(                      )");
                panel.mvaddstr(14, 0, " )                    ( ");
                panel.mvaddstr(15, 0, "(                      )");
                panel.mvaddstr(16, 0, " )                    ( ");
                panel.mvaddstr(17, 0, "(                      )");
                panel.mvaddstr(18, 0, " )                    ( ");
                panel.mvaddstr(19, 0, "(                      )");
                panel.mvaddstr(20, 0, " )                    ( ");
                panel.mvaddstr(21, 0, " '-._.-=-._.-=-._.-=-.- ");
            }
        }
    }

    fn create_panel_3(app_win: &pancurses::Window) -> pancurses::Window {
        let panel = match app_win.subwin(22, 24, 0, 48) {
            Ok(win) => win,
            Err(code) => panic!("pancurses subwin function failed w/ result code {}", code),
        };
        panel.mvaddstr(00, 0, "                       +");
        panel.mvaddstr(01, 0, "          +--------+    ");
        panel.mvaddstr(02, 0, " Next     :[][][][]:    ");
        panel.mvaddstr(03, 0, " Block:   :[][][][]:    ");
        panel.mvaddstr(04, 0, "          :[][][][]:    ");
        panel.mvaddstr(05, 0, "          :[][][][]:    ");
        panel.mvaddstr(06, 0, "          +--------+    ");
        panel.mvaddstr(07, 0, "                        ");
        panel.mvaddstr(08, 0, " Current Level:       0 ");
        panel.mvaddstr(09, 0, " Lines Cleared:       0 ");
        panel.mvaddstr(10, 0, "                        ");
        panel.mvaddstr(11, 0, " Four-Liners:         0 ");
        panel.mvaddstr(12, 0, " Three-Liners:        0 ");
        panel.mvaddstr(13, 0, " Two-Liners:          0 ");
        panel.mvaddstr(14, 0, " One-Liners:          0 ");
        panel.mvaddstr(15, 0, "                        ");
        panel.mvaddstr(16, 0, " Points:              0 ");
        panel.mvaddstr(17, 0, " Top-Score:           0 ");
        panel.mvaddstr(18, 0, "                        ");
        panel.mvaddstr(19, 0, " https://github.com/..  ");
        panel.mvaddstr(20, 0, "       /cgrage/rustris  ");
        panel.mvaddstr(21, 0, "                       +");
        return panel;
    }
}
