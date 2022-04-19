use std::io;
use std::collections::HashMap;
use rand::Rng;
/*
  COMP10068 Secure Programming
  Assignment 2 - T2 2022
  Banner No.: B00371286
*/

fn main() {
  //MAX and MIN constraint values
  const MAX_VALUE :i32 = 9;
  const MIN_VALUE :i32 = 1;
  let mut playing = true;  
  //Outer Game Loop
  while playing {
    println!("\n### Welcome to Naughts and Crosses! ###\n");
    //Initalise Game variables
    let mut game_over : bool = false;    
    let mut turn_number : i32 = 1;
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
    let rng_num :i32 = rand::thread_rng().gen_range(MIN_VALUE..MAX_VALUE);
    println!("Deciding who goes first, random number drawn was '{}'",rng_num);
    if rng_num % 2 == 0 {
      player_turn = true; 
      player_symbol.push_str("X"); cpu_symbol.push_str("O");
      println!("Even Number - Player goes first! You are {}'s",player_symbol);
    } else if !player_turn{      
      player_symbol.push_str("O"); cpu_symbol.push_str("X");
      println!("Odd Number - CPU goes first! You are {}'s",player_symbol);
    }    
    //Display the empty board before game start to show the numbered grid
    println!();
    display_board(&board);
    //Inner Main Game Loop
    loop{          
      if turn_number > MAX_VALUE{game_over = !game_over};
      if game_over {        
        println!{"The game was a draw!\n"};
        display_board(&board);
        break;
      }
      println!("\n# Turn {} #\n", turn_number);
      if player_turn {        
        let player_choice :i32 = player_move(MAX_VALUE, MIN_VALUE, &board, &choices_made);
        choices_made.push(player_choice);
        board.insert(player_choice, &player_symbol);
        println!(); display_board(&board);                                           
      } else if !player_turn{        
        let cpu_choice :i32 = cpu_move(MAX_VALUE, MIN_VALUE, &choices_made);
        choices_made.push(cpu_choice);
        board.insert(cpu_choice, &cpu_symbol);
        display_board(&board);        
      }                
      if turn_number > 4 {game_over = check_board(&board);} //Can only get set of 3 on turn 5 or after
      if game_over{
        match player_turn{
          true => {println!("\nCongratulations, you won this game!\n");display_board(&board);break;},
          false => {println!("\nThe CPU won, better luck next game!\n");display_board(&board); break;},
        }
      }      
      turn_number=&turn_number+1;
      player_turn = !&player_turn;
    };    
    //Game Over - print game moves
    println!("\n*** Game Over! ***\n");
    show_game_moves(&board, &choices_made, &player_symbol);  
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
    if board.get(&n).unwrap() == &empty_val {
      print!("{} ",n)}
    else {      
      print!("{} ",board.get(&n).unwrap())
    }
    if n % 3 == 0 {print!("\n")};      
  }  
  println!();
}

fn player_move(max_value :i32, min_value :i32, board: &HashMap<i32,&str>, choices_made : &Vec<i32>) ->i32 {
  loop{               
    println!("Pick a number between {} and {}.", min_value,max_value);          
    let mut player_choice = String::new();     
    io::stdin().read_line(&mut player_choice)
      .expect("Failed to read line");
    let choice: i32 = match player_choice.trim().parse() {
      Ok(num) => num,      
      Err(_)  => {println!("Incorrect value, please try again\n"); continue;}, 
    };
    match choice {                  
      g if g > max_value => {println!("Out of Bounds: Too big!\n"); continue;},   
      g if g < min_value => {println!("Out of Bounds: Too small!\n"); continue;}, 
      _ => {
        if choices_made.contains(&choice) { 
          println!("\n'{}' has already been picked, try again.\n",choice);
          display_board(&board);
          continue;                
        } else {
          return choice;                          
        } 
      }
    }
  }
} 

fn cpu_move(max_value :i32, min_value :i32, choices_made : &Vec<i32>) -> i32 {
  loop{
    let cpu_choice = rand::thread_rng().gen_range(min_value..(max_value+1));  //+1 necessary otherwise it excludes picking 9          
    if choices_made.contains(&cpu_choice) { 
      //println!("'{}' has already been picked, picking again.",cpu_guess);   //debug println
      continue;                
    } else {
      println!("CPU choice is {}.\n",cpu_choice);
      return cpu_choice;                   
    }         
  }
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
    if board.get(&7).unwrap() == board.get(&8).unwrap() && board.get(&8).unwrap() == board.get(&9).unwrap(){three_found = true} //third row
  };
  return three_found;  
}

fn show_game_moves(board : &HashMap<i32, &str>,choices_made : &Vec<i32>, player_symbol : &String){
  let mut choice_no =1;
  println!("Game Moves:");
  for x in choices_made {
    if board.get(&x).unwrap() == player_symbol{
      println!("  Move No.{}> {} by Player",choice_no,x);  
    } else {
      println!("  Move No.{}> {} by CPU",choice_no,x);  
    }
    choice_no=&choice_no+1;
  }  
}
