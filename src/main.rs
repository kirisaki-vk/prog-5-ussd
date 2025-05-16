use crate::menus::main_menu::MainMenu;
use crate::models::menu::Menu;
use std::io;
use std::io::Write;

mod menus;
mod models;

fn main() {
    let mut current_menu: Box<dyn Menu> = MainMenu::new_boxed();
    loop {
        clear_screen();
        current_menu.present_menu();
        println!("--------------------------------------------");
        println!("Page précedent: 00");
        println!("Menu principale: **");
        println!("Quitter l'application: ##");
        print!("Entrer votre choix: ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice = choice.trim();

        match choice {
            "00" => {
                if let Some(prev_menu) = current_menu.previous_menu() {
                    current_menu = prev_menu
                }
            }
            "##" => return,
            "**" => current_menu = MainMenu::new_boxed(),
            _ => {
                if let Ok(index) = choice.parse::<usize>() {
                    if let Some(next_menu) = current_menu.navigate(index - 1) {
                        current_menu = next_menu;
                    }
                }
            }
        }
    }
}
fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
