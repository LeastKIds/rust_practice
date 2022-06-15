fn main() {
    // println!("Hello, world!");
    // // todo!("Display the message by using println!() macro");
    // println!("The first letter of the English alphabet is {} and the last letter is {}", 'A', 'Z');

    // let a_number;
    // let a_word = "Ten";
    //
    // a_number = 10;
    //
    // println!("The number is {}", a_number);
    // println!("The word is {}", a_word);

    // let mut a_number = 10;
    // println!("The number is {}", a_number);
    //
    // a_number = 15;
    // println!("The number is {}", a_number);

    // let shadow_num = 5;
    // let shadow_num = shadow_num + 5;
    //
    // let shadow_num = shadow_num * 2;
    //
    // println!("this number is {}", shadow_num);

    // let number: u32 = "14";
    // println!("The number is {}", number);

    // let number_64 = 4.0;
    // let number_32: f32 = 5.0;

    // println!("1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}", 1u32 + 2, 8i32 - 5, 15 * 3);
    // println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);
    //
    // let number = -1;

    // let string_2: &str = "ace";
    // let string_1 = "miley ";

    // let tuple_e = ('E', 5i32, true);
    // println!("{}",tuple_e.2)

    // struct Student { name: String, level: u8, remote: bool }
    //
    // let user_2 = Student

    // Define a tuple struct
//     #[derive(Debug)]
//     struct KeyPress(String, char);
//
//     // Define a classic struct
//     #[derive(Debug)]
//     struct MouseClick { x: i64, y: i64 }
//
//     // Define the WebEvent enum variants to use the data from the structs
// // and a boolean type for the page Load variant
//     #[derive(Debug)]
//     enum WebEvent { WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress) }
//
// // Instantiate a MouseClick struct and bind the coordinate values
//     let click = MouseClick { x: 100, y: 250 };
//     println!("Mouse click location: {}, {}", click.x, click.y);
//
// // Instantiate a KeyPress tuple and bind the key values
//     let keys = KeyPress(String::from("Ctrl+"), 'N');
//     println!("\nKeys pressed: {}{}", keys.0, keys.1);
//
// // Instantiate WebEvent enum variants
// // Set the boolean page Load value to true
//     let we_load = WebEvent::WELoad(true);
// // Set the WEClick variant to use the data in the click struct
//     let we_click = WebEvent::WEClick(click);
// // Set the WEKeys variant to use the data in the keys tuple
//     let we_key = WebEvent::WEKeys(keys);
//
// // Print the values in the WebEvent enum variants
// // Use the {:#?} syntax to display the enum structure and data in a readable form
//     println!("\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", we_load, we_click, we_key);

    println!("{}", divide_by_5(10))

}

fn divide_by_5(num: u32) -> u32 {
    return num / 5
}