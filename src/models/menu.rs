pub trait Menu {
    fn present_menu(&self) {
        println!("{}", self.title());
        println!("--------------------------------------------");
        for (index, item) in self.items().iter().enumerate() {
            println!("{}. {}", index + 1, item.name);
        }
    }
    fn items(&self) -> &[MenuItem];
    fn navigate(&self, _index: usize) -> Option<Box<dyn Menu>> {
        todo!()
    }
    fn previous_menu(&self) -> Option<Box<dyn Menu>>;
    fn title(&self) -> &str;
}

#[allow(dead_code)]
pub struct MenuItem {
    pub name: String,
    pub menu: Option<Box<dyn Menu>>,
}
