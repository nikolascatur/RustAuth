use std::io::{self, Write};

use crate::{
    config::postgress_setup,
    model::{
        session::UserSession,
        users::{CreateUser, LoginUser, Logout},
    },
    service::user_service::UserService,
};

struct Menu {
    mode_menu: MenuMode,
    user_service: UserService,
    user_session: Option<UserSession>,
}

enum MenuMode {
    Main,
    Login,
    Home,
    Exit,
}

impl Menu {
    pub async fn new() -> Self {
        let db = postgress_setup().await.unwrap();
        // .unwrap_or_else(|err| println!("{}", err.to_string()));
        Self {
            mode_menu: MenuMode::Main,
            user_service: UserService::new(&db),
            user_session: None,
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
            MenuMode::Home => self.home_screen().await,
            MenuMode::Exit => self.exit_screen().await,
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

    async fn home_screen(&mut self) {
        self.clear_terminal();
        println!("Selamat Datang Di Home");
        println!("1. Info User");
        println!("2. Logout");
        let mut select_menu = String::new();
        println!("Pilih Menu Anda ");
        io::stdin()
            .read_line(&mut select_menu)
            .expect("Failed read menu");
        let select_menu = select_menu.trim();
        match select_menu {
            "1" => {}
            "2" => {
                println!("session --> {:?}", &self.user_session);
                if let Some(ses) = &self.user_session {
                    println!("sessionvaluee - {}", ses.session_token);
                    let logout = Logout {
                        session_id: ses.session_token.clone(),
                    };
                    let result = self
                        .user_service
                        .logout(&logout)
                        .await
                        .unwrap_or_else(|_| false);
                    if result {
                        self.mode_menu = MenuMode::Main;
                    } else {
                        println!("Failed to logout");
                    }
                }
            }
            _ => {}
        }
    }

    async fn exit_screen(&mut self) {
        self.clear_terminal();
        println!("Unauthorized user");
        let mut select_menu = String::new();
        println!("Pilih Menu Anda ");
        io::stdin()
            .read_line(&mut select_menu)
            .expect("Failed read menu");
    }

    async fn login_menu(&mut self) {
        self.clear_terminal();
        println!("Masukkan Alamat Email Anda");
        let mut password = String::new();
        let mut email = String::new();
        let mut name = String::new();
        io::stdin()
            .read_line(&mut email)
            .expect("Failed Read email");
        let is_user_exist = self.user_service.is_user_exist(&email).await;
        match is_user_exist {
            Ok(n) => {
                if n {
                    println!("Masukkan Password Anda");
                    io::stdin()
                        .read_line(&mut password)
                        .expect("Failed Read Password");
                    let login_user = LoginUser {
                        email: email,
                        password: password,
                    };
                    let login_result = self.user_service.login(&login_user).await;
                    match login_result {
                        Ok(ses) => {
                            self.mode_menu = MenuMode::Home;
                            self.user_session = Some(ses);
                        }
                        Err(_) => self.mode_menu = MenuMode::Exit,
                    }
                } else {
                    println!("Masukkaan Nama Anda");
                    io::stdin().read_line(&mut name).expect("faield read name");
                    println!("Masukkan Password Anda");
                    io::stdin()
                        .read_line(&mut password)
                        .expect("Failed Read Password");
                    let user = CreateUser {
                        email: email,
                        name: name,
                        password: password,
                    };
                    let result_create = self.user_service.create_user(user).await;
                    match result_create {
                        Ok(session) => {
                            self.user_session = Some(session);
                            self.mode_menu = MenuMode::Home
                        }
                        Err(_) => self.mode_menu = MenuMode::Exit,
                    }
                }
            }
            Err(err) => {
                println!("Error {}", err);
            }
        }
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
