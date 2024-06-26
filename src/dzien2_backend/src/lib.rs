use std::cell::{RefCell, RefMut};


thread_local! {
    static WPISY: RefCell<Vec<String>> = RefCell::default(); 
}

#[ic_cdk::query]
fn greet(name: String, last_name: i8) -> String {
    format!("Hello, {} {}!", name, last_name)
}

#[ic_cdk::update]
fn dodaj_wpis(wpis: String){
    WPISY.with(|wpisy: &RefCell<Vec<String>>| {
        wpisy.borrow_mut().push(wpis)
    });
}

#[ic_cdk::query]
fn odczytaj_wpisy() -> Vec<String>{
    WPISY.with(|wpisy: &RefCell<Vec<String>>| {
        wpisy.borrow().clone()
    })
}

#[ic_cdk::update]
fn usun_wpis(id_wpis: usize){
    WPISY.with(|wpisy: &RefCell<Vec<String>>| {
        wpisy.borrow_mut().remove(id_wpis)
    });
}
#[ic_cdk::update]
fn edytuj_wpis(id_wpis: usize, new_wpis: String){
    WPISY.with(|wpisy: &RefCell<Vec<String>>| {
        let mut binding: RefMut<Vec<String>> = wpisy.borrow_mut();
        let wpis: Option<&mut String> = binding.get_mut(id_wpis);
        let prev_wpis: &mut String = wpis.unwrap();
        *prev_wpis = new_wpis;
    });
}