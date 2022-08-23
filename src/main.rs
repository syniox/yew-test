use yew::{
    events::{
        InputEvent,
        KeyboardEvent,
        MouseEvent,
    },
    function_component, classes, 
    Properties, html, Callback, Component, Context, Html
};
extern crate web_sys;

enum Msg {
    Input(InputEvent),
    Press(KeyboardEvent),
    Click(char)
}

struct Comp(String);


const KEYBOARD_0: [char; 10] = ['Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P'];
const KEYBOARD_1: [char; 9] = ['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L'];
const KEYBOARD_2: [char; 7] = ['Z', 'X', 'C', 'V', 'B', 'N', 'M'];

#[derive(Properties, PartialEq)]
pub struct KeybrButtonProps {
    pub on_press: Callback<MouseEvent>,
    pub character: char // needed key_col
}

#[function_component(KeybrButton)]
pub fn keybr_button(props: &KeybrButtonProps) -> Html {
    html! {
        //<button style={format!("background-color:{}","yellow")}>
        <button class={"keybr-button"}>
        {
            //props.character.or(Some(' ')).unwrap()
            props.character
        }
        </button>
    }
}

// Keyboard viewing function
fn keyarr2html<T: yew::Component>(arr: &'static [char], ctx: &Context<T>) -> Html
where <T as yew::Component>::Message: From <Msg> { // ??????
    html! {
        <div class={classes!("keybr_row")}>
        {
            arr.iter().map(|c| html! {
                <KeybrButton character={*c} on_press={
                    &ctx.link().callback(|_: MouseEvent| Msg::Click(*c))
                }/>
            }).collect::<Html>()
        }
        </div>
    }
}

impl Component for Comp {

    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self(String::new())
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg{
            Msg::Input(input) => {
                if let Some(s) = input.data() {
                    log::info!("printed: {:?}", s);
                } else {
                    log::info!("nothing typed");
                }
                match input.input_type().as_str() {
                    "insertText" => self.0 = input.data().unwrap(),
                    "deleteContentBackward" => self.0 = "Backspace".to_string(),
                    e => panic!("Unknown type: {}",e)
                }
            }
            Msg::Press(event) => {
                if event.key() == "Enter" {
                    self.0 = "Enter".to_string();
                }
            }
            Msg::Click(c) => {
                log::info!("Clicked: {}", c);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = &ctx.link().callback(|event: InputEvent| {
            Msg::Input(event)
        });
        let onkeypress = &ctx.link().callback(|event: KeyboardEvent| {
            Msg::Press(event)
        });
        let keybr_r0 = keyarr2html(&KEYBOARD_0, ctx);
        let keybr_r1 = keyarr2html(&KEYBOARD_1, ctx);
        let keybr_r2 = keyarr2html(&KEYBOARD_2, ctx);

        html! {
            <h1 style="text-align:center">
            <div class={"board"}>
            {
                (0..6).map(|_| html! {
                    <div class={"row"}>
                    {
                        (0..5).map(|_id| html! {
                            <div class={classes!("tile")}/>
                        }).collect::<Html>()
                    }
                    </div>
                }).collect::<Html>()
            }
            </div>
            {
              (0..5).map(|_| html! {
              <input type="text" {oninput} {onkeypress}/>
               }).collect::<Html>()
            }
            <p>{ keybr_r0 }</p>
            <p>{ keybr_r1 }</p>
            <p>{ keybr_r2 }</p>
            <p>{ &(self.0) }</p>
            </h1>
        }
    }
}

fn main(){
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Comp>();
}
