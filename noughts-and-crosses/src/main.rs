use std::io;
use std::{thread, time};
use std::collections::HashMap;
use rand::Rng;
/*
  COMP10068 Secure Programming
  Assignment 2 - T2 2022
*/

fn main() {
  //MAX and MIN contraint values
  const MAX_VALUE :i32 = 9;
  const MIN_VALUE :i32 = 1;
  let mut playing = true;  
  //Outer Game Loop
  while playing {
    println!("\n### Welcome to Naughts and Crosses! ###\n");
    //Initalise Game variables
    let mut game_over : bool = false;    
    let mut turn_num = 1;
    let mut board: HashMap<i32, &str> = HashMap::from([
      (1,""), (2,""), (3,""),
      (4,""), (5,""), (6,""),
      (7,""), (8,""), (9,""),
    ]);    
    let mut choices_made: Vec<i32> = Vec::new();
    let mut player_symbol = String::new();   
    let mut cpu_symbol = String::new();
    //Decide who is X and O, X starts the game
    //Even - Player, Odd - CPU
    let mut player_turn : bool = false;
    let rng_num = rand::thread_rng().gen_range(MIN_VALUE..MAX_VALUE);
    println!("Deciding who goes first, random number drawn was '{}'",rng_num);
    if rng_num % 2 == 0 {
      player_turn = true; 
      player_symbol.push_str("X"); cpu_symbol.push_str("O");
      println!("Even Number - Player goes first! You are {}'s",player_symbol);
    } else if !player_turn{      
      player_symbol.push_str("O"); cpu_symbol.push_str("X");
      println!("Odd Number - CPU goes first! You are {}'s",player_symbol);
    }
    //Small pause to let player see who goes first
    println!("\nGame starting in...");
    for n in(1..4).rev() {
      println!("{}...",n);
      thread::sleep(time::Duration::from_secs(1));      
    }
    //Inner Main Game Loop
    loop{          
      if turn_num == 10{game_over = !game_over};
      if game_over {
        print!{"The game was a draw!\n"};
        break;
      }
      println!("\n# Turn {} #\n", turn_num);
      if player_turn {
        loop{     
          display_board(&board);
          println!("Pick a number between {} and {}!", MIN_VALUE,MAX_VALUE);
          let mut guess = String::new();     
          io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
          let choice: i32 = match guess.trim().parse() {
            Ok(num) => num,      
            Err(_)  => {println!("Incorrect value, please try again\n"); continue;}, 
          };
          match choice {                  
            g if g > MAX_VALUE => {println!("Out of Bounds: Too big!\n"); continue;},   
            g if g < MIN_VALUE => {println!("Out of Bounds: Too small!\n"); continue;}, 
            _ => {
              if choices_made.contains(&choice) { 
                println!("\n'{}' has already been picked, try again.",choice);
                continue;                
              } else {
                choices_made.push(choice);
                board.insert(choice, &player_symbol);
                break;  
              } 
            }
          }
        }
      } else if !player_turn{
        loop{
          let cpu_choice = rand::thread_rng().gen_range(MIN_VALUE..(MAX_VALUE+1));  //+1 necessary otherwise it excludes picking 9          
          if choices_made.contains(&cpu_choice) { 
            //println!("'{}' has already been picked, picking again.",cpu_guess);   //debug println
            continue;                
          } else {
            println!("CPU choice is {}\n",cpu_choice);
            choices_made.push(cpu_choice);
            board.insert(cpu_choice, &cpu_symbol);
            break;  
          }         
        }
      }
      println!();      
      if turn_num > 4 {game_over = check_board(&board);} //Can only get set of 3 on turn 5 or after
      if game_over{
        match player_turn{
          true => {display_board(&board);println!("\nCongratulations, you won this game!"); break;},
          false => {display_board(&board);println!("\nThe CPU won, better luck next game!"); break;},
        }
      }      
      turn_num=turn_num+1;
      player_turn = !player_turn;
    };    
    //Game Over - print game moves
    println!("\n*** Game Over! ***\n");
    let mut choice_no =1;
    println!("Game Moves:");
    for x in &choices_made {
      if board.get(&x).unwrap() == &player_symbol{
        println!("  Move No.{}> {} by Player",choice_no,x);  
      } else {
        println!("  Move No.{}> {} by CPU",choice_no,x);  
      }
      choice_no=&choice_no+1;
    }    
    //Retry Game Loop  
    loop{    
      println!("\nDo you wish to try again? (Y/N)");
      let mut play_again = String::new();
      io::stdin().read_line(&mut play_again)
        .expect("Failed to read line");
      match play_again.trim(){
        c if c.to_lowercase().eq("n") => {playing=false; break;},
        c if c.to_lowercase().eq("y") => {println!("Setting up new round..."); break;},            
        _  => {println!("Not Valid, try again"); continue;}, 
      };
    };    
  }
  //End Message
  print!("\nThanks for Playing!");
}

fn display_board(board : &HashMap<i32, &str>) {
  let empty_val = "";
  for n in 1..10 {
    let grid_value = board.get(&n).unwrap();
    if grid_value == &empty_val {print!("{} ",n)}
    else {      
      print!("{} ",&grid_value)
    }
    if n % 3 == 0 {print!("\n")};      
  }  
  println!();
}

fn check_board(board : &HashMap<i32, &str>) -> bool {
  let empty_val = "";
  let mut three_found : bool= false;
  if board.get(&1).unwrap() != &empty_val{
    if board.get(&1).unwrap() == board.get(&2).unwrap() && board.get(&2).unwrap() == board.get(&3).unwrap(){three_found = true} //first row 
    else if board.get(&1).unwrap() == board.get(&4).unwrap() && board.get(&4).unwrap() == board.get(&7).unwrap(){three_found = true} //first column
    else if board.get(&1).unwrap() == board.get(&5).unwrap() && board.get(&5).unwrap() == board.get(&9).unwrap(){three_found = true} // diagonal left to right
  }
  if board.get(&3).unwrap() != &empty_val{
    if board.get(&3).unwrap() == board.get(&5).unwrap() && board.get(&5).unwrap() == board.get(&7).unwrap(){three_found = true} //diagonal right to left
    else if board.get(&3).unwrap() == board.get(&6).unwrap() && board.get(&6).unwrap() == board.get(&9).unwrap(){three_found = true} //third column    
  }
  if board.get(&2).unwrap() != &empty_val{
    if board.get(&2).unwrap() == board.get(&5).unwrap() && board.get(&5).unwrap() == board.get(&8).unwrap(){three_found = true} //middle column
  }
  if board.get(&4).unwrap() != &empty_val{
    if board.get(&4).unwrap() == board.get(&5).unwrap() && board.get(&5).unwrap() == board.get(&6).unwrap(){three_found = true} //middle row
  }  
  if board.get(&7).unwrap() != &empty_val{
    if board.get(&7).unwrap() == board.get(&8).unwrap() && board.get(&8).unwrap() == board.get(&9).unwrap(){three_found = true} //bottom row
  };
  return three_found;  
}
