use crate::models::menu::{Menu, MenuItem};

pub struct MainMenu {
    menu_items: Vec<MenuItem>,
    title: String
}

impl MainMenu {
    pub fn new() -> Self {
        MainMenu {
            title: "Airtel".to_string(),
            menu_items: vec![
                MenuItem {
                    name: "Recharger".to_string(),
                    menu: None,
                },
                MenuItem {
                    name: "Envoyer argent".to_string(),
                    menu: None
                },
                MenuItem {
                    name: "Payment".to_string(),
                    menu: None,
                },
                MenuItem {
                    name: "Retirer argent".to_string(),
                    menu: None
                },
                MenuItem {
                    name: "Banque".to_string(),
                    menu: None,
                },
                MenuItem {
                    name: "Mon compte/Mot de passe".to_string(),
                    menu: None
                },
                MenuItem {
                    name: "Les Offres".to_string(),
                    menu: None,
                },
                MenuItem {
                    name: "Services/Factures".to_string(),
                    menu: None
                }
            ]
        }
    }
    
    pub fn new_boxed() -> Box<Self> {
        Box::new(Self::new())
    }
}

impl Menu for MainMenu {
    fn items(&self) -> &[MenuItem] {
        &self.menu_items
    }
     fn previous_menu(&self) -> Option<Box<dyn Menu>> {
        None   
     }

    fn title(&self) -> &str {
        self.title.as_str()
    }
}