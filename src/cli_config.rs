use std::io::{self, Write};

use crate::{
    config::{init_db_pool, postgress_setup},
    service::user_service::UserService,
};

struct Menu {
    mode_menu: MenuMode,
    sub_menu: Option<i32>,
    user_service: UserService,
}

enum MenuMode {
    Main,
    Login,
    CheckNo,
    Create,
}

impl Menu {
    pub async fn new() -> Self {
        let db = postgress_setup().await.unwrap();
        Self {
            mode_menu: MenuMode::Main,
            sub_menu: None,
            user_service: UserService::new(&db),
        }
    }

    pub fn clear_terminal(&self) {
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().unwrap();
        println!()
    }
    pub async fn show_menu(&mut self) {
        match self.mode_menu {
            MenuMode::Main => self.main_menu(),
            MenuMode::Login => self.login_menu(),
            MenuMode::Create => self.create_menu(),
            _ => {}
        }
    }

    fn parse_int(&self, menu: String) -> i32 {
        let number = menu.trim().parse::<i32>().unwrap_or_else(|_| -1);
        number
    }

    fn main_menu(&mut self) {
        self.clear_terminal();
        println!("1. Login");
        println!("2. Exit");
        let mut select_menu = String::new();
        println!("Pilih Menu Anda ");
        io::stdin()
            .read_line(&mut select_menu)
            .expect("Failed read menu");
        let number = self.parse_int(select_menu);
        self.go_next_main(&number);
    }

    fn login_menu(&self) {
        self.clear_terminal();
        println!("Masukkan Alamat Email Anda");
        let mut email = String::new();
        io::stdin()
            .read_line(&mut email)
            .expect("Failed Read email");
    }

    fn create_menu(&self) {
        self.clear_terminal();
    }

    fn go_next_main(&mut self, menu: &i32) {
        let mn = menu;
        match mn {
            1 => {
                self.mode_menu = MenuMode::Login;
                self.show_menu();
            }
            _ => {
                println!("Bye Bye...")
            }
        }
    }
}

pub async fn run_cli() {
    let mut menu = Menu::new().await;
    menu.show_menu();
}
