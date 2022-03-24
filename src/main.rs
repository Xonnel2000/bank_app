////////////////////////////////////////////////////////////////////////////
/// this is a test bank account
use grep::check_useraccout;
use std::io;

fn main() {
    // the program start with a loop question to pick an answer from the option
    loop {
        println!("YOUR BANK ACCOUNT!");
        println!(
            "PLEASE SELECT YOUR OPTION
        OPTION 1: login  
        OPTION 2: exit the program 
        "
        );
        println!("Please input your option.");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: i32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", option);

        /*       match accountop1 {
            Bankoption {
                option1: option, ..
            } => println!("{}", option),
            Bankoption {
                option2: _option, ..
            } => println!("option 2"),
            Bankoption {
                option3: _option, ..
            } => {
                println!("You win!");
                break;
            }
        }*///end of match

        match option {
            1 => {
                /*   println!("Please input your username.");

                let mut username = String::new();

                io::stdin()
                    .read_line(&mut username)
                    .expect("Failed to read line");*/

                println!("Please input your password.");

                let mut password = String::new();
                io::stdin()
                    .read_line(&mut password)
                    .expect("Failed to read line");
                println!("You password is {}", password);

                //i use a lib with this function
                check_useraccout(password);
            }
            2 => {
                println!("Thank you for using are service");
                break;
            }
            _ => println!("Please a valid option"),
        } //end of match
    } //end of loop
}

/* fn check_option(option: String) {
    let mut accountop1 = Bankoption {
        option1: 1,
        option2: 2,
        option3: 3,
    };
    while accountop1.option3 == option.trim() as i32 {
        if accountop1.option1 == 1 {
            println!("Please input your username.");

            let mut username = String::new();

            io::stdin()
                .read_line(&mut username)
                .expect("Failed to read line");

            println!("Please input your password.");

            let mut password = String::new();
            io::stdin()
                .read_line(&mut password)
                .expect("Failed to read line");
            println!("You usrname is: {} and password is {}", username, password);
            checkUseraccout(username, password);
        }
    }
} */
