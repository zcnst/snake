mod random;
mod snake;

use js_sys::Function;
use snake::SnakeGame;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, UnwrapThrowExt, JsCast};
use web_sys::{console, window, HtmlElement, HtmlDivElement, KeyboardEvent};
use crate::snake::Direction; 

thread_local! {
    static GAME: Rc<RefCell<SnakeGame>> = Rc::new(RefCell::new(SnakeGame::new(15, 15)));
 
    static TICK_CLOSURE: Closure<dyn FnMut()> = Closure::wrap(Box::new({
        || {
            GAME.with(|game| game.borrow_mut().tick());
            render();
        }
    }) as Box<dyn FnMut()>);

    static HANDLE_KEYDOWN: Closure<dyn FnMut(KeyboardEvent)> = Closure::wrap(Box::new({
        |evt: KeyboardEvent| {
            GAME.with(|game| {
                let direction = match &evt.key()[..] {
                    "ArrowUp" => Some(Direction::Up),
                    "ArrowRight" => Some(Direction::Right),
                    "ArrowLeft" => Some(Direction::Left),
                    "ArrowDown" => Some(Direction::Down),
                    _ => None,
                };

                if let Some(direction) = direction {
                    game.borrow_mut().change_direction(direction);
                }
            });
        }
    }) as Box<dyn FnMut(KeyboardEvent)>); 
}

#[wasm_bindgen(start)]
pub fn main(){
    console::log_1(&"Starting..".into());

    TICK_CLOSURE.with(|tick_closure| {
        window()
            .unwrap_throw()
            .set_interval_with_callback_and_timeout_and_arguments_0(
            tick_closure.as_ref().dyn_ref::<Function>().unwrap_throw(), 
            250,
        )
        .unwrap_throw();
    });

    HANDLE_KEYDOWN.with(|handle_keydown| {
        window()
            .unwrap_throw()
            .add_event_listener_with_callback(
                "keydown", 
                handle_keydown.as_ref().dyn_ref::<Function>().unwrap_throw()
            ).unwrap_throw();
    }); 

    render();
}

pub fn render() { 
    let document = window()
        .unwrap_throw()
        .document()
        .unwrap_throw();

    let root_container = document
        .get_element_by_id("root")
        .unwrap_throw()
        .dyn_into::<HtmlElement>()
        .unwrap_throw();
    
        root_container.set_inner_html("");

        let height = GAME.with(|game| game.borrow().height);
        let width = GAME.with(|game| game.borrow().width);

        root_container
            .style()
            .set_property("display", "inline-grid")
            .unwrap_throw();
        root_container
            .style()
            .set_property(
                "grid-template", 
                &format!("repeat({}, auto) / repeat({}, auto)", width, height)
            )
            .unwrap_throw();


        for y in 0..height {
            for x in 0..width {
                let pos = (x, y);
                let field_element = document
                    .create_element("div")
                    .unwrap_throw()
                    .dyn_into::<HtmlDivElement>()
                    .unwrap_throw();
                
                field_element.set_inner_text({
                    if pos == GAME.with(|game| game.borrow().food) {
                        "🍕"
                    } else if GAME.with(|game| game.borrow().snake.contains(&pos)) {
                        "🟩"
                    } else {
                        "⬜️"
                    }
                });
                
                root_container.append_child(&field_element).unwrap_throw();
            }
        }
}