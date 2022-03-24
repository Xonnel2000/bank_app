use std::io;

////////////////////////////////////////////////////////////////////////////
/// this is a test bank account

//use struct to hold password just for testing
struct Useraccount {
    password: String,
}

// create a cons var to hold the total balance
const ACCOUNT_BALANCE: i64 = 2_000_000;

//the check function to check if password is correct and log you in
pub fn check_useraccout(password: String) {
    //print!("{},{}", username, password);

    let user1 = Useraccount {
        password: String::from("1234"),
    };
    //let dusername = user1.usernamer;
    let dpassword = user1.password;
    //print!("{},{}", dusername, dpassword);

    //check is the password is correct if correct it log u in if not it ask again to enter correct
    //password
    if dpassword == password.trim() {
        println!("thank you for logging ");

        loop {
            println!(
                "PLEASE SELECT YOUR OPTION
            OPTION 1: check your balance   
            OPTION 2: withdraw 
            OPTION 2: exit "
            );
            println!("Please input your options.");
            let mut pullmoni_option = String::new();

            io::stdin()
                .read_line(&mut pullmoni_option)
                .expect("Failed to read line");

            let pullmoni_option: i32 = match pullmoni_option.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            // use the match option to match the user input
            match pullmoni_option {
                1 => {
                    println!(
                        "your accout balance is {}. do you want to contiune",
                        ACCOUNT_BALANCE
                    );
                    println!(
                        "PLEASE SELECT YOUR OPTION
                    OPTION 1: yes   
                    OPTION 2: no 
                    "
                    );

                    let mut yes_ornooption = String::new();

                    io::stdin()
                        .read_line(&mut yes_ornooption)
                        .expect("Failed to read line");

                    let yes_ornooption: i32 = match yes_ornooption.trim().parse() {
                        Ok(num) => num,
                        Err(_) => continue,
                    };

                    if yes_ornooption == 1 {
                        continue;
                    }
                    break;
                }
                2 => {
                    println!("Please enter the amount you want to redraw.");

                    let mut pullmoni = String::new();

                    io::stdin()
                        .read_line(&mut pullmoni)
                        .expect("Failed to read line");

                    let pullmoni: i32 = match pullmoni.trim().parse() {
                        Ok(num) => num,
                        Err(_) => continue,
                    };

                    if pullmoni as i64 > ACCOUNT_BALANCE {
                        println!(
                            "sorry you do not have amount of money  your current balace is {}",
                            ACCOUNT_BALANCE
                        );
                    } else {
                        let new_balance: i64 = ACCOUNT_BALANCE - pullmoni as i64;

                        println!(
                            "your new balance is {} money pull is {}",
                            new_balance, pullmoni
                        );
                    }
                }
                _ => println!("Please a valid option"),
            } //end of match
        } //end of loop
    } else {
        println!("username and password is incorrect");
    }
}
