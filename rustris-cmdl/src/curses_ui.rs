use crate::board::Board;
use crate::common::{CellVal, Stats, UiState, UserInput};
use std::time::{Duration, Instant};

const ONE_SECOND: Duration = Duration::from_secs(1);

#[derive(Debug)]
pub struct UI {
    screen: pancurses::Window,
    app_win: pancurses::Window,
    panel_1: pancurses::Window,
    panel_2: pancurses::Window,
    panel_3: pancurses::Window,
    fps_count: i32,
    fps_time: Instant,
    fps_value: i32,
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
        screen.nodelay(true);
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
            fps_count: 0,
            fps_time: Instant::now(),
            fps_value: -1,
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

    pub fn read_user_input(&self) -> UserInput {
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
            Some(pancurses::Input::Character('n')) => UserInput::Reset,
            _ => UserInput::NoInput,
        }
    }

    pub fn draw(&mut self, board: &Board, stats: &Stats) {
        let now = Instant::now();
        if now.duration_since(self.fps_time) > ONE_SECOND {
            self.fps_value = self.fps_count;
            self.fps_count = 0;
            self.fps_time = now;
        }

        self.draw_board(&board);
        self.draw_active_block(&board);
        self.draw_next_block(&board);
        self.draw_stats(&stats);

        self.app_win.touch();
        self.app_win.refresh();

        self.fps_count += 1;
    }

    fn draw_board(&self, board: &Board) {
        for y in 0..board.height() {
            for x in 0..board.width() {
                let val = board.cell_value_at_board(x, y);
                self.panel_2.mv(1 + y, 2 + 2 * x);
                self.panel_2.printw(self.cell_string(&val));
            }
        }
    }

    fn draw_stats(&self, stats: &Stats) {
        self.panel_3.mvprintw(6, 16, format!("{:7}", stats.cleared));
        self.panel_3.mvprintw(8, 16, format!("{:7}", stats.clr_cmb_4));
        self.panel_3.mvprintw(9, 16, format!("{:7}", stats.clr_cmb_3));
        self.panel_3.mvprintw(10, 16, format!("{:7}", stats.clr_cmb_2));
        self.panel_3.mvprintw(11, 16, format!("{:7}", stats.clr_cmb_1));
        self.panel_3.mvprintw(20, 21, format!("{:2}", self.fps_value));
    }

    fn draw_active_block(&self, board: &Board) {
        for y in 0..4 {
            for x in 0..4 {
                let val = board.cell_value_at_active_block(x, y);
                match val {
                    CellVal::Free => (),
                    CellVal::OutOfBoard => (),
                    _ => {
                        let (px, py) = board.active_block_pos_to_board_pos(x, y);
                        self.panel_2.mv(1 + py, 2 + 2 * px);
                        self.panel_2.printw("[]");
                    }
                }
            }
        }
    }

    fn draw_next_block(&self, board: &Board) {
        for y in 0..3 {
            for x in 0..4 {
                let val = board.cell_value_at_next_block(x, y);
                self.panel_3.mv(1 + y, 10 + 2 * x);
                self.panel_3.printw(self.cell_string(&val));
            }
        }
    }

    fn cell_string(&self, val: &CellVal) -> &str {
        return match val {
            CellVal::Free => "  ",
            CellVal::Color0 => "{}",
            CellVal::Color1 => "{}",
            CellVal::Color2 => "{}",
            CellVal::Color3 => "{}",
            CellVal::Color4 => "{}",
            CellVal::Color5 => "{}",
            CellVal::Color6 => "{}",
            CellVal::Color7 => "{}",
            CellVal::Color8 => "{}",
            CellVal::Color9 => "{}",
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
        panel.mvaddstr(08, 0, " Please find the source ");
        panel.mvaddstr(09, 0, " code in github.        ");
        panel.mvaddstr(10, 0, "                        ");
        panel.mvaddstr(11, 0, " Key Mappings:          ");
        panel.mvaddstr(12, 0, "     q - Quit           ");
        panel.mvaddstr(13, 0, "     a - Move left      ");
        panel.mvaddstr(14, 0, "     d - Move right     ");
        panel.mvaddstr(15, 0, "     s - Move down      ");
        panel.mvaddstr(16, 0, "     w - Drop           ");
        panel.mvaddstr(17, 0, "  Left - Rotate         ");
        panel.mvaddstr(18, 0, " Right - Rotate         ");
        panel.mvaddstr(19, 0, " Space - Change UI      ");
        panel.mvaddstr(20, 0, "     n - Reset          ");
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
        panel.mvaddstr(01, 0, " Next     ########      ");
        panel.mvaddstr(02, 0, " Block:   ########      ");
        panel.mvaddstr(03, 0, "          ########      ");
        panel.mvaddstr(04, 0, "                        ");
        panel.mvaddstr(05, 0, " Current Level: ####### ");
        panel.mvaddstr(06, 0, " Lines Cleared: ####### ");
        panel.mvaddstr(07, 0, "                        ");
        panel.mvaddstr(08, 0, " Four-Liners:   ####### ");
        panel.mvaddstr(09, 0, " Three-Liners:  ####### ");
        panel.mvaddstr(10, 0, " Two-Liners:    ####### ");
        panel.mvaddstr(11, 0, " One-Liners:    ####### ");
        panel.mvaddstr(12, 0, "                        ");
        panel.mvaddstr(13, 0, " Points:        ####### ");
        panel.mvaddstr(14, 0, " Top-Score:     ####### ");
        panel.mvaddstr(15, 0, "                        ");
        panel.mvaddstr(16, 0, " github.com/            ");
        panel.mvaddstr(17, 0, "   cgrage/rustris       ");
        panel.mvaddstr(18, 0, "                        ");
        panel.mvaddstr(19, 0, "                        ");
        panel.mvaddstr(20, 0, "                FPS: ## ");
        panel.mvaddstr(21, 0, "                       +");
        return panel;
    }
}
