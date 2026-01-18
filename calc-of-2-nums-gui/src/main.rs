use fltk::{app, button::Button, frame::Frame, input::Input, menu::Choice, prelude::*, window::Window};

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(200, 200, 540, 300, "Calculator");

    let mut input_num1 = Input::new(120, 50, 80, 35, "");
    let mut input_num2 = Input::new(280, 50, 80, 35, "");

    let mut input_action = Choice::new(220, 50, 40, 35, "");
    input_action.add_choice("+");
    input_action.add_choice("-");
    input_action.add_choice("*");
    input_action.add_choice("/");
    input_action.set_value(0);

    let mut result_frame = Frame::new(55, 150, 200, 40, "Result: ");
    
    let mut calc_button = Button::new(50, 240, 100, 35, "Get result");

    wind.end();
    wind.show();

    let mut num1 = input_num1.clone();
    let mut num2 = input_num2.clone();
    let mut action = input_action.clone();
    let mut res_frame_clone = result_frame.clone();

    calc_button.set_callback(move |_| {
        let num1_str = num1.value();
        let num2_str = num2.value();
        let act_index = action.value();
        let actions_list = ["+", "-", "*", "/"];
        let actions = actions_list[act_index as usize];

        match (num1_str.parse::<f64>(), num2_str.parse::<f64>()) {
            (Ok(num1), Ok(num2)) => {
                let result = match actions {
                    "+" => num1 + num2,
                    "-" => num1 - num2,
                    "*" => num1 * num2,
                    "/" => {if num1 != 0.0 && num2 != 0.0 {
                        num1 / num2
                    }
                    else { f64::NAN }
                },
                    _ => 0.0,
                };

                if result.is_nan() {
                    res_frame_clone.set_label("Error: division by 0");
                } else {
                    res_frame_clone.set_label(&format!("{} {} {} = {:.2}", num1, actions, num2, result));
                }
            }

            _ => {
                res_frame_clone.set_label("   Error: Something went wrong, or you just stupid");
            }
        }
    });


    app.run().unwrap();
}