use std::io::{self, Write};

use tokio::signal::{self, unix::Signal};

use crate::{
    config::{init_db_pool, postgress_setup},
    service::user_service::{self, UserService},
};

struct Menu {
    mode_menu: MenuMode,
    sub_menu: Option<i32>,
    user_service: UserService,
}

enum MenuMode {
    Main,
    Login,
    InputPassword,
    Create,
}

impl Menu {
    pub async fn new() -> Self {
        let db = postgress_setup().await.unwrap();
        // .unwrap_or_else(|err| println!("{}", err.to_string()));
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
            MenuMode::Main => self.main_menu().await,
            MenuMode::Login => self.login_menu().await,
            MenuMode::Create => self.create_menu().await,
            MenuMode::InputPassword => self.input_password(),
            _ => {}
        }
    }

    fn parse_int(&self, menu: String) -> i32 {
        let number = menu.trim().parse::<i32>().unwrap_or_else(|_| -1);
        number
    }

    async fn main_menu(&mut self) {
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

    async fn login_menu(&mut self) {
        self.clear_terminal();
        println!("Masukkan Alamat Email Anda");
        let mut email = String::new();
        io::stdin()
            .read_line(&mut email)
            .expect("Failed Read email");
        let is_user_exist = self.user_service.is_user_exist(email).await;
        match is_user_exist {
            Ok(n) => {
                if n {
                    self.mode_menu = MenuMode::InputPassword
                } else {
                    self.mode_menu = MenuMode::Create
                }
            }
            Err(err) => {
                println!("Error {}", err);
            }
        }
    }

    fn input_password(&self) {
        self.clear_terminal();
        println!("Masukkan Password Anda");
    }

    async fn create_menu(&mut self) {
        self.clear_terminal();
        println!("Buat Nama User")
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
    loop {
        menu.show_menu().await;
    }
    // signal::ctrl_c().await.unwrap();
}
