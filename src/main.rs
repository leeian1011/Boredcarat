mod cards;
use std::{io::prelude::*, process::exit};

fn main() {
    let mut deck = cards::Deck::new();

    deck.shuffle();

    let mut player: [cards::Card; 2] = [deck.draw(), cards::Card::placeholder()];
    println!("Player\n{} \r", player[0]);

    std::thread::sleep(std::time::Duration::from_secs(1));

    let mut banker: [cards::Card; 2] = [deck.draw(), cards::Card::placeholder()];
    println!("Banker\n{} \r\n\n", banker[0]);

    std::thread::sleep(std::time::Duration::from_secs(1));

    player[1] = deck.draw();
    println!("Player\n{} {} \r", player[0], player[1]);

    std::thread::sleep(std::time::Duration::from_secs(1));

    banker[1] = deck.draw();
    println!("Banker\n{} {} \r\n\n", banker[0], banker[1]);

    std::thread::sleep(std::time::Duration::from_secs(1));

    let mut player_sum = Into::<i32>::into(player[0].value) + Into::<i32>::into(player[1].value);
    if player_sum >= 10 {
        player_sum = player_sum - 10;
        if player_sum >= 10 {
            player_sum = player_sum - 10;
        }
    }

    let mut banker_sum = Into::<i32>::into(banker[0].value) + Into::<i32>::into(banker[1].value);
    if banker_sum >= 10 {
        banker_sum = banker_sum - 10;
        if banker_sum >= 10 {
            banker_sum = banker_sum - 10;
        }
    }

    println!("Player: {}, Banker: {}\n\n", player_sum, banker_sum);

    if banker_sum == 9 {
        println!("Banker Wins");
        exit(0);
    } else if player_sum == 9 {
        println!("Player Wins");
        exit(0);
    }

    if player_sum == 8 && banker_sum < player_sum {
        println!("Player Wins");
        exit(0);
    } else if banker_sum == 8 && player_sum < banker_sum {
        println!("Banker Wins");
        exit(0);
    }

    if player_sum < 6 && banker_sum < 8 {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let new_card = deck.draw();
        println!("Player\n{} {} \n {} \r", player[0], player[1], new_card);
        player_sum = player_sum + Into::<i32>::into(new_card.value);
        if player_sum >= 10 {
            player_sum = player_sum - 10;
            if player_sum >= 10 {
                player_sum = player_sum - 10;
            }
        }

        if banker_sum > 5 {
            if player_sum > banker_sum {
                println!("Player Wins");
            } else if banker_sum > player_sum {
                println!("Banker Wins");
            } else {
                println!("Tie");
            }

            exit(0);
        } else {
            std::thread::sleep(std::time::Duration::from_secs(1));
            let new_card = deck.draw();
            println!("Banker\n{} {} \n {} \r", banker[0], banker[1], new_card);
            banker_sum = banker_sum + Into::<i32>::into(new_card.value);
            if banker_sum >= 10 {
                banker_sum = banker_sum - 10;
                if banker_sum >= 10 {
                    banker_sum = banker_sum - 10;
                }
            }

            println!("Player: {}, Banker: {}", player_sum, banker_sum);
            if player_sum > banker_sum {
                println!("Player Wins");
            } else if banker_sum > player_sum {
                println!("Banker Wins");
            } else {
                println!("Tie");
            }

            exit(0);
        }
    }

    if banker_sum < 6 {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let new_card = deck.draw();
        println!("Banker\n{} {} \n {} \r", banker[0], banker[1], new_card);
        banker_sum = banker_sum + Into::<i32>::into(new_card.value);
        if banker_sum >= 10 {
            banker_sum = banker_sum - 10;
            if banker_sum >= 10 {
                banker_sum = banker_sum - 10;
            }
        }

        println!("Player: {}, Banker: {}", player_sum, banker_sum);

        if player_sum > banker_sum {
            println!("Player Wins");
        } else if banker_sum > player_sum {
            println!("Banker Wins");
        } else {
            println!("Tie");
        }

        exit(0);
    }

    if banker_sum > player_sum {
        println!("Banker Wins");
    } else if player_sum > banker_sum {
        println!("Player Wins");
    } else {
        println!("Tie");
    }

    exit(0);
}
