use stdweb;
use stdweb::unstable::TryInto;
use stdweb::web::{
    IEventTarget,
    document,
};

use stdweb::web::event::{
    IEvent,
    IKeyboardEvent,
    KeypressEvent,
};

use stdweb::web::html_element::InputElement;

// Shamelessly stolen from webplatform's TodoMVC example.
macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

pub fn main() {
    stdweb::initialize();

    let message: InputElement = document().query_selector(".the-input").unwrap().try_into().unwrap();
    message.add_event_listener(enclose!( (message) move |event: KeypressEvent| {
        if event.key() == "Enter" {
            event.prevent_default();

            let msg: String = message.value().try_into().unwrap();
            js! {
                alert( @{msg} );
            }
        }
    }));

    stdweb::event_loop();
}
