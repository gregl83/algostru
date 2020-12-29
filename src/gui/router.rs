use std::{
    rc::Rc,
    cell::RefCell,
};

use super::store::Store;
use super::screen::Screenable;
use std::borrow::BorrowMut;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Route {
    Title,
    Dashboard,
}

pub struct Router {
    store: Rc<RefCell<Store>>,
    routes: Vec<Route>,
    screens: Vec<Box<dyn Screenable>>,
}

impl Router {
    pub fn new(store: Rc<RefCell<Store>>, route_map: Vec<(Route, Box<dyn Screenable>)>) -> Self {
        let mut routes = vec![];
        let mut screens = vec![];

        for (route, screen) in route_map {
            routes.push(route);
            screens.push(screen);
        }

        Router {
            store,
            routes,
            screens
        }
    }

    fn find_screen(&mut self, route: Route) -> &mut Box<dyn Screenable> {
        let index = self.routes.iter().position(|r| *r == route).unwrap();
        &mut self.screens[index]
    }

    pub fn get_screen(&mut self) -> &mut Box<dyn Screenable> {
        let store_ref = Rc::clone(&self.store);
        let store = store_ref.borrow();
        match *store {
            Store {welcomed: false, ..} => self.find_screen(Route::Title).borrow_mut(),
            Store {welcomed: true, ..} => self.find_screen(Route::Dashboard).borrow_mut(),
        }
    }
}