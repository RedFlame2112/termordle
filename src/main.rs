extern crate termion;
use clap::{App, Arg};
use std::fs::File;
use std::io::Read;
use std::io::{stdin, stdout, Write};

use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use rand::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum GERROR{
    BadLength,
    InvalidWord,
}
#[derive(Debug, Eq, PartialEq, Clone)]
enum HInfo{
    Correct, //is char (in) correct pos
    Partial, //char within word but not in right pos
    Miss, //miss
    None,
}
impl std::fmt::Display for GERROR {
    fn fmt(&self, form: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GERROR::BadLength => write!(form, "Invalid length"),
            GERROR::InvalidWord => write!(form, "Not in list"),
        }
    }
}

struct GState {
    valid: Vec<String>,
    guess_bin: Vec<String>,
    curr_guess: String,
    word: String,
    tries_max: u16,
    l_error: Option<GERROR>,
    any_word: bool,
}
impl GState {
    pub fn new(word: String, valid: Vec<String>, any_word: bool,) -> GState {
        GState {
            valid,
            guess_bin: Vec::new(),
            curr_guess: String::new(),
            word,
            tries_max: 6,
            l_error: None,
            any_word,
        }
    }
    pub fn won(&self) -> bool {
        match self.guess_bin.last() {
            Some(l_guess) => l_guess == &self.word,
            None => false,
        }
    }
    pub fn get_hits(&self, guess_pos: usize) -> Vec<HInfo> {
        let mut hits = Vec::new();
        let guess = self.guess_bin.get(guess_pos).unwrap();
        for (i, ch) in guess.chars().enumerate() {
            if ch == self.word.chars().nth(i).unwrap(){
                hits.push(HInfo::Correct);
            } else if self.word.contains(ch) {
                hits.push(HInfo::Partial);
            } else {
                hits.push(HInfo::Miss);
            }
        } hits
    }
    pub fn move_back(&mut self) {
        if self.curr_guess.chars().count() > 0 {
            self.curr_guess.pop();
        }
    }
    pub fn affirm(&mut self) {
        let res = self.make_guess(self.curr_guess.clone());
        match res {
            Ok(_) => {
                self.reset_err();
            }
            Err(err) => {
                self.set_last_err(err);
            }
        };
        self.curr_guess = String::new();
    }
    pub fn push_char(&mut self, c: char) {
        if self.curr_guess.chars().count() < self.word.chars().count() {
            self.curr_guess.push(c.to_lowercase().next().unwrap());
        }
    }
    fn make_guess(&mut self, your_guess: String) -> Result<bool, GERROR> {
        if your_guess.chars().count() != self.word.chars().count() {
            return Err(GERROR::BadLength);
        }
        if !self.any_word && !self.valid.contains(&your_guess){
            return Err(GERROR::InvalidWord);
        } self.guess_bin.push(your_guess);
        Ok(self.won())
    }
    fn set_last_err(&mut self, error: GERROR) {
        self.l_error = Some(error);
    }
    fn reset_err(&mut self) {
        self.l_error = None;
    }

}

/* !====================== BEGIN func RENDER GSTATE =========================== */
fn render_gstate(g_state: &GState) {
    let mut output = stdout().into_raw_mode().unwrap();
    writeln!(output, "{}{}", termion::clear::All, termion::cursor::Hide).unwrap();
    let w = g_state.word.chars().count() as u16;
    let h = g_state.tries_max as u16;
    let top = 4;
    let left = 10;
    for i in 0..h {
        write!(output, "{}{}", termion::cursor::Goto(left, top + i * 2 - 1), (0..(w * 2 + 1)).map(|_| "-").collect::<String>()).unwrap();

        let lin_guess: String; //GET: guess of an entire line or a string of blanks (_)
        if i < g_state.guess_bin.len() as u16 {
            lin_guess = g_state.guess_bin[i as usize].clone();
        } else if i == g_state.guess_bin.len() as u16 {
            let mut curr = g_state.curr_guess.clone();
            while curr.chars().count() < w as usize {
                curr.push('_');
            }
            lin_guess = curr;
        } else {
            lin_guess = (0..w).map(|_| "_").collect::<String>();
        }

        //get all correct pos in line
        let line_corrects: Vec<HInfo>;
        if (i as usize) < g_state.guess_bin.len() {
            line_corrects = g_state.get_hits(i as usize);
        } else {
            line_corrects = vec![HInfo::None; w as usize];
        }

        for j in 0..w {
            //push every letter into its own cell
            write!(output, "{}|", termion::cursor::Goto(left + j * 2, top + i * 2),).unwrap();

            //ooo fancy colors
            let h_info = line_corrects.get(j as usize).unwrap();
            //letter is a hit!
            if h_info == &HInfo::Correct {
                write!(output, "{}{}", color::Bg(color::Magenta), color::Fg(color::Black)).unwrap(); 
                //BG: magenta, LETTER: black
            }
            //letter is partially correct!
            if h_info == &HInfo::Partial {
                write!(output, "{}{}", color::Bg(color::Cyan), color::Fg(color::Black)).unwrap(); 
                //BG: Cyan, LETTER: black
            }
            if h_info == &HInfo::Miss {
                write!(output, "{}{}", color::Bg(color::Black), color::Fg(color::White)).unwrap(); 
                //BG: black, LETTER: white (basically it will not change much)
            }
            if h_info == &HInfo::None {
                write!(output, "{}{}", color::Bg(color::Reset), color::Fg(color::Reset)).unwrap(); 
                //BG: black, LETTER: white (basically it will not change much)
            }
            write!(output, "{}{}{}", lin_guess.chars().nth(j as usize).unwrap(), color::Bg(color::Reset), color::Fg(color::White)).unwrap();
        } writeln!(output, "|").unwrap(); //exit cell
    }
    // print out the error below the game board
    match g_state.l_error {
        Some(err) => {
            writeln!(output, "{}{}", termion::cursor::Goto(left, top + h * 2 + 1), format!("{}", err)).unwrap();
        } None => (),
    }
}
/* !====================== END func RENDER GSTATE =========================== */


/* !====================== BUILD GSTATE LOOP =========================== */
fn g_loop(mut g_state: GState) {
    let mut input = stdin().keys();
    let mut output  = stdout().into_raw_mode().unwrap();
    'g_loop: while g_state.guess_bin.len() < 6 {
        render_gstate(&g_state);
        'input_loop: loop {
            let k = input.next().unwrap().unwrap();
            match k {
                Key::Esc => break 'g_loop,
                Key::Backspace => g_state.move_back(),
                Key::Char('\n') => {g_state.affirm(); break 'input_loop;}
                Key::Char(ch) => g_state.push_char(ch), _=> (),
            }
            output.flush().unwrap();
            render_gstate(&g_state);
        }

        match g_state.l_error {
            None => {
                if g_state.won() {
                    println!("Congratulations! You won!");
                    break;
                }
            } _=> (),
        }
    }

    render_gstate(&g_state);
    writeln!(output, "{}", termion::cursor::Show).unwrap();
    if !g_state.won() {
        println!("Aww poop! Better luck next time. The word was: {}", g_state.word);
    }
    if g_state.won() {
        println!("Congratulations! You won!");
    }
}

fn init(any_word: bool, wfile: Option<&str>) -> GState {
    //load: valid wlist from file
    let mut wlist = Vec::new();
    let word;

    match wfile {
        Some(file) => {
            let mut file = File::open(file).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            wlist = contents.split('\n').map(|s| s.trim().to_string().to_lowercase()).collect();
            let mut rnjesus = rand::thread_rng();
            let i = rnjesus.gen::<usize>() % wlist.len();
            word = wlist[i].clone();
        }
        None => {
            let sol_str = include_str!("../lists/solutions.txt");
            for line in sol_str.lines() {
                wlist.push(line.to_string().to_lowercase());
            }
            let mut rnjesus = rand::thread_rng();
            let i = rnjesus.gen::<usize>() % wlist.len();
            word = wlist[i].clone();

            let valid_word_str = include_str!("../lists/valid_words.txt");
            for line in valid_word_str.lines() {
                wlist.push(line.to_string().to_lowercase());
            }
        }
    }
    let g_state = GState::new(word, wlist, any_word);
    g_state
}
fn main() {
    let matches = App::new("termordle").version("0.1.0").about("An open-source wordle app for terminal written in Rust").arg(Arg::new("any-guess").short('g').long("any-guess").takes_value(false).help("Allow any word to be guessed")).arg(Arg::new("wfile").short('w').long("word-file").takes_value(true).help("Use a word list from a file")).get_matches();
    let g_state = init(matches.is_present("any-guess"), matches.value_of("wfile"));
    g_loop(g_state)
}
