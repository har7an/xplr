use termion::event::Key as TermionKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Key {
    Backspace,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    BackTab,
    Delete,
    Insert,
    Return,
    Space,
    Tab,
    Escape,

    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,

    CtrlZero,
    CtrlOne,
    CtrlTwo,
    CtrlThree,
    CtrlFour,
    CtrlFive,
    CtrlSix,
    CtrlSeven,
    CtrlEight,
    CtrlNine,

    AltZero,
    AltOne,
    AltTwo,
    AltThree,
    AltFour,
    AltFive,
    AltSix,
    AltSeven,
    AltEight,
    AltNine,

    ShiftZero,
    ShiftOne,
    ShiftTwo,
    ShiftThree,
    ShiftFour,
    ShiftFive,
    ShiftSix,
    ShiftSeven,
    ShiftEight,
    ShiftNine,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    CtrlA,
    CtrlB,
    CtrlC,
    CtrlD,
    CtrlE,
    CtrlF,
    CtrlG,
    CtrlH,
    CtrlI,
    CtrlJ,
    CtrlK,
    CtrlL,
    CtrlM,
    CtrlN,
    CtrlO,
    CtrlP,
    CtrlQ,
    CtrlR,
    CtrlS,
    CtrlT,
    CtrlU,
    CtrlV,
    CtrlW,
    CtrlX,
    CtrlY,
    CtrlZ,

    AltA,
    AltB,
    AltC,
    AltD,
    AltE,
    AltF,
    AltG,
    AltH,
    AltI,
    AltJ,
    AltK,
    AltL,
    AltM,
    AltN,
    AltO,
    AltP,
    AltQ,
    AltR,
    AltS,
    AltT,
    AltU,
    AltV,
    AltW,
    AltX,
    AltY,
    AltZ,

    ShiftA,
    ShiftB,
    ShiftC,
    ShiftD,
    ShiftE,
    ShiftF,
    ShiftG,
    ShiftH,
    ShiftI,
    ShiftJ,
    ShiftK,
    ShiftL,
    ShiftM,
    ShiftN,
    ShiftO,
    ShiftP,
    ShiftQ,
    ShiftR,
    ShiftS,
    ShiftT,
    ShiftU,
    ShiftV,
    ShiftW,
    ShiftX,
    ShiftY,
    ShiftZ,

    CtrlShiftA,
    CtrlShiftB,
    CtrlShiftC,
    CtrlShiftD,
    CtrlShiftE,
    CtrlShiftF,
    CtrlShiftG,
    CtrlShiftH,
    CtrlShiftI,
    CtrlShiftJ,
    CtrlShiftK,
    CtrlShiftL,
    CtrlShiftM,
    CtrlShiftN,
    CtrlShiftO,
    CtrlShiftP,
    CtrlShiftQ,
    CtrlShiftR,
    CtrlShiftS,
    CtrlShiftT,
    CtrlShiftU,
    CtrlShiftV,
    CtrlShiftW,
    CtrlShiftX,
    CtrlShiftY,
    CtrlShiftZ,

    AltShiftA,
    AltShiftB,
    AltShiftC,
    AltShiftD,
    AltShiftE,
    AltShiftF,
    AltShiftG,
    AltShiftH,
    AltShiftI,
    AltShiftJ,
    AltShiftK,
    AltShiftL,
    AltShiftM,
    AltShiftN,
    AltShiftO,
    AltShiftP,
    AltShiftQ,
    AltShiftR,
    AltShiftS,
    AltShiftT,
    AltShiftU,
    AltShiftV,
    AltShiftW,
    AltShiftX,
    AltShiftY,
    AltShiftZ,

    Plus,
    Minus,
    Backtick,
    Tilde,
    Underscore,
    Equals,
    Semicolon,
    Colon,
    SingleQuote,
    DoubleQuote,
    ForwardSlash,
    BackSlash,
    Dot,
    Comma,
    QuestionMark,

    NotSupported,
}

impl Key {
    pub fn from_termion_event(key: TermionKey) -> Self {
        match key {
            TermionKey::Backspace => Key::Backspace,
            TermionKey::Left => Key::Left,
            TermionKey::Right => Key::Right,
            TermionKey::Up => Key::Up,
            TermionKey::Down => Key::Down,
            TermionKey::Home => Key::Home,
            TermionKey::End => Key::End,
            TermionKey::PageUp => Key::PageUp,
            TermionKey::PageDown => Key::PageDown,
            TermionKey::BackTab => Key::BackTab,
            TermionKey::Delete => Key::Delete,
            TermionKey::Insert => Key::Insert,
            TermionKey::Char('\n') => Key::Return,
            TermionKey::Char(' ') => Key::Space,
            TermionKey::Char('\t') => Key::Tab,
            TermionKey::Esc => Key::Escape,

            TermionKey::Char('0') => Key::Zero,
            TermionKey::Char('1') => Key::One,
            TermionKey::Char('2') => Key::Two,
            TermionKey::Char('3') => Key::Three,
            TermionKey::Char('4') => Key::Four,
            TermionKey::Char('5') => Key::Five,
            TermionKey::Char('6') => Key::Six,
            TermionKey::Char('7') => Key::Seven,
            TermionKey::Char('8') => Key::Eight,
            TermionKey::Char('9') => Key::Nine,

            TermionKey::Ctrl('0') => Key::CtrlZero,
            TermionKey::Ctrl('1') => Key::CtrlOne,
            TermionKey::Ctrl('2') => Key::CtrlTwo,
            TermionKey::Ctrl('3') => Key::CtrlThree,
            TermionKey::Ctrl('4') => Key::CtrlFour,
            TermionKey::Ctrl('5') => Key::CtrlFive,
            TermionKey::Ctrl('6') => Key::CtrlSix,
            TermionKey::Ctrl('7') => Key::CtrlSeven,
            TermionKey::Ctrl('8') => Key::CtrlEight,
            TermionKey::Ctrl('9') => Key::CtrlNine,

            TermionKey::Alt('0') => Key::AltZero,
            TermionKey::Alt('1') => Key::AltOne,
            TermionKey::Alt('2') => Key::AltTwo,
            TermionKey::Alt('3') => Key::AltThree,
            TermionKey::Alt('4') => Key::AltFour,
            TermionKey::Alt('5') => Key::AltFive,
            TermionKey::Alt('6') => Key::AltSix,
            TermionKey::Alt('7') => Key::AltSeven,
            TermionKey::Alt('8') => Key::AltEight,
            TermionKey::Alt('9') => Key::AltNine,

            TermionKey::Char('a') => Key::A,
            TermionKey::Char('b') => Key::B,
            TermionKey::Char('c') => Key::C,
            TermionKey::Char('d') => Key::D,
            TermionKey::Char('e') => Key::E,
            TermionKey::Char('f') => Key::F,
            TermionKey::Char('g') => Key::G,
            TermionKey::Char('h') => Key::H,
            TermionKey::Char('i') => Key::I,
            TermionKey::Char('j') => Key::J,
            TermionKey::Char('k') => Key::K,
            TermionKey::Char('l') => Key::L,
            TermionKey::Char('m') => Key::M,
            TermionKey::Char('n') => Key::N,
            TermionKey::Char('o') => Key::O,
            TermionKey::Char('p') => Key::P,
            TermionKey::Char('q') => Key::Q,
            TermionKey::Char('r') => Key::R,
            TermionKey::Char('s') => Key::S,
            TermionKey::Char('t') => Key::T,
            TermionKey::Char('u') => Key::U,
            TermionKey::Char('v') => Key::V,
            TermionKey::Char('w') => Key::W,
            TermionKey::Char('x') => Key::X,
            TermionKey::Char('y') => Key::Y,
            TermionKey::Char('z') => Key::Z,

            TermionKey::Ctrl('a') => Key::CtrlA,
            TermionKey::Ctrl('b') => Key::CtrlB,
            TermionKey::Ctrl('c') => Key::CtrlC,
            TermionKey::Ctrl('d') => Key::CtrlD,
            TermionKey::Ctrl('e') => Key::CtrlE,
            TermionKey::Ctrl('f') => Key::CtrlF,
            TermionKey::Ctrl('g') => Key::CtrlG,
            TermionKey::Ctrl('h') => Key::CtrlH,
            TermionKey::Ctrl('i') => Key::CtrlI,
            TermionKey::Ctrl('j') => Key::CtrlJ,
            TermionKey::Ctrl('k') => Key::CtrlK,
            TermionKey::Ctrl('l') => Key::CtrlL,
            TermionKey::Ctrl('m') => Key::CtrlM,
            TermionKey::Ctrl('n') => Key::CtrlN,
            TermionKey::Ctrl('o') => Key::CtrlO,
            TermionKey::Ctrl('p') => Key::CtrlP,
            TermionKey::Ctrl('q') => Key::CtrlQ,
            TermionKey::Ctrl('r') => Key::CtrlR,
            TermionKey::Ctrl('s') => Key::CtrlS,
            TermionKey::Ctrl('t') => Key::CtrlT,
            TermionKey::Ctrl('u') => Key::CtrlU,
            TermionKey::Ctrl('v') => Key::CtrlV,
            TermionKey::Ctrl('w') => Key::CtrlW,
            TermionKey::Ctrl('x') => Key::CtrlX,
            TermionKey::Ctrl('y') => Key::CtrlY,
            TermionKey::Ctrl('z') => Key::CtrlZ,

            TermionKey::Alt('a') => Key::AltA,
            TermionKey::Alt('b') => Key::AltB,
            TermionKey::Alt('c') => Key::AltC,
            TermionKey::Alt('d') => Key::AltD,
            TermionKey::Alt('e') => Key::AltE,
            TermionKey::Alt('f') => Key::AltF,
            TermionKey::Alt('g') => Key::AltG,
            TermionKey::Alt('h') => Key::AltH,
            TermionKey::Alt('i') => Key::AltI,
            TermionKey::Alt('j') => Key::AltJ,
            TermionKey::Alt('k') => Key::AltK,
            TermionKey::Alt('l') => Key::AltL,
            TermionKey::Alt('m') => Key::AltM,
            TermionKey::Alt('n') => Key::AltN,
            TermionKey::Alt('o') => Key::AltO,
            TermionKey::Alt('p') => Key::AltP,
            TermionKey::Alt('q') => Key::AltQ,
            TermionKey::Alt('r') => Key::AltR,
            TermionKey::Alt('s') => Key::AltS,
            TermionKey::Alt('t') => Key::AltT,
            TermionKey::Alt('u') => Key::AltU,
            TermionKey::Alt('v') => Key::AltV,
            TermionKey::Alt('w') => Key::AltW,
            TermionKey::Alt('x') => Key::AltX,
            TermionKey::Alt('y') => Key::AltY,
            TermionKey::Alt('z') => Key::AltZ,

            TermionKey::Char('A') => Key::ShiftA,
            TermionKey::Char('B') => Key::ShiftB,
            TermionKey::Char('C') => Key::ShiftC,
            TermionKey::Char('D') => Key::ShiftD,
            TermionKey::Char('E') => Key::ShiftE,
            TermionKey::Char('F') => Key::ShiftF,
            TermionKey::Char('G') => Key::ShiftG,
            TermionKey::Char('H') => Key::ShiftH,
            TermionKey::Char('I') => Key::ShiftI,
            TermionKey::Char('J') => Key::ShiftJ,
            TermionKey::Char('K') => Key::ShiftK,
            TermionKey::Char('L') => Key::ShiftL,
            TermionKey::Char('M') => Key::ShiftM,
            TermionKey::Char('N') => Key::ShiftN,
            TermionKey::Char('O') => Key::ShiftO,
            TermionKey::Char('P') => Key::ShiftP,
            TermionKey::Char('Q') => Key::ShiftQ,
            TermionKey::Char('R') => Key::ShiftR,
            TermionKey::Char('S') => Key::ShiftS,
            TermionKey::Char('T') => Key::ShiftT,
            TermionKey::Char('U') => Key::ShiftU,
            TermionKey::Char('V') => Key::ShiftV,
            TermionKey::Char('W') => Key::ShiftW,
            TermionKey::Char('X') => Key::ShiftX,
            TermionKey::Char('Y') => Key::ShiftY,
            TermionKey::Char('Z') => Key::ShiftZ,

            TermionKey::Ctrl('A') => Key::CtrlShiftA,
            TermionKey::Ctrl('B') => Key::CtrlShiftB,
            TermionKey::Ctrl('C') => Key::CtrlShiftC,
            TermionKey::Ctrl('D') => Key::CtrlShiftD,
            TermionKey::Ctrl('E') => Key::CtrlShiftE,
            TermionKey::Ctrl('F') => Key::CtrlShiftF,
            TermionKey::Ctrl('G') => Key::CtrlShiftG,
            TermionKey::Ctrl('H') => Key::CtrlShiftH,
            TermionKey::Ctrl('I') => Key::CtrlShiftI,
            TermionKey::Ctrl('J') => Key::CtrlShiftJ,
            TermionKey::Ctrl('K') => Key::CtrlShiftK,
            TermionKey::Ctrl('L') => Key::CtrlShiftL,
            TermionKey::Ctrl('M') => Key::CtrlShiftM,
            TermionKey::Ctrl('N') => Key::CtrlShiftN,
            TermionKey::Ctrl('O') => Key::CtrlShiftO,
            TermionKey::Ctrl('P') => Key::CtrlShiftP,
            TermionKey::Ctrl('Q') => Key::CtrlShiftQ,
            TermionKey::Ctrl('R') => Key::CtrlShiftR,
            TermionKey::Ctrl('S') => Key::CtrlShiftS,
            TermionKey::Ctrl('T') => Key::CtrlShiftT,
            TermionKey::Ctrl('U') => Key::CtrlShiftU,
            TermionKey::Ctrl('V') => Key::CtrlShiftV,
            TermionKey::Ctrl('W') => Key::CtrlShiftW,
            TermionKey::Ctrl('X') => Key::CtrlShiftX,
            TermionKey::Ctrl('Y') => Key::CtrlShiftY,
            TermionKey::Ctrl('Z') => Key::CtrlShiftZ,

            TermionKey::Alt('A') => Key::AltShiftA,
            TermionKey::Alt('B') => Key::AltShiftB,
            TermionKey::Alt('C') => Key::AltShiftC,
            TermionKey::Alt('D') => Key::AltShiftD,
            TermionKey::Alt('E') => Key::AltShiftE,
            TermionKey::Alt('F') => Key::AltShiftF,
            TermionKey::Alt('G') => Key::AltShiftG,
            TermionKey::Alt('H') => Key::AltShiftH,
            TermionKey::Alt('I') => Key::AltShiftI,
            TermionKey::Alt('J') => Key::AltShiftJ,
            TermionKey::Alt('K') => Key::AltShiftK,
            TermionKey::Alt('L') => Key::AltShiftL,
            TermionKey::Alt('M') => Key::AltShiftM,
            TermionKey::Alt('N') => Key::AltShiftN,
            TermionKey::Alt('O') => Key::AltShiftO,
            TermionKey::Alt('P') => Key::AltShiftP,
            TermionKey::Alt('Q') => Key::AltShiftQ,
            TermionKey::Alt('R') => Key::AltShiftR,
            TermionKey::Alt('S') => Key::AltShiftS,
            TermionKey::Alt('T') => Key::AltShiftT,
            TermionKey::Alt('U') => Key::AltShiftU,
            TermionKey::Alt('V') => Key::AltShiftV,
            TermionKey::Alt('W') => Key::AltShiftW,
            TermionKey::Alt('X') => Key::AltShiftX,
            TermionKey::Alt('Y') => Key::AltShiftY,
            TermionKey::Alt('Z') => Key::AltShiftZ,

            TermionKey::Char('+') => Key::Plus,
            TermionKey::Char('-') => Key::Minus,
            TermionKey::Char('`') => Key::Backtick,
            TermionKey::Char('~') => Key::Tilde,
            TermionKey::Char('_') => Key::Underscore,
            TermionKey::Char('=') => Key::Equals,
            TermionKey::Char(';') => Key::Semicolon,
            TermionKey::Char(':') => Key::Colon,
            TermionKey::Char('\'') => Key::SingleQuote,
            TermionKey::Char('"') => Key::DoubleQuote,
            TermionKey::Char('/') => Key::ForwardSlash,
            TermionKey::Char('\\') => Key::BackSlash,
            TermionKey::Char('.') => Key::Dot,
            TermionKey::Char(',') => Key::Comma,
            TermionKey::Char('?') => Key::QuestionMark,

            _ => Key::NotSupported,
        }
    }
}