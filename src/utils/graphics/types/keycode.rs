use strum_macros::{EnumString, Display};

#[derive(Debug, Display, EnumString)]
pub enum KeyCode {
    // Alphanumeric (a–z)
    #[strum(serialize = "a")] KeyCodeA,
    #[strum(serialize = "b")] KeyCodeB,
    #[strum(serialize = "c")] KeyCodeC,
    #[strum(serialize = "d")] KeyCodeD,
    #[strum(serialize = "e")] KeyCodeE,
    #[strum(serialize = "f")] KeyCodeF,
    #[strum(serialize = "g")] KeyCodeG,
    #[strum(serialize = "h")] KeyCodeH,
    #[strum(serialize = "i")] KeyCodeI,
    #[strum(serialize = "j")] KeyCodeJ,
    #[strum(serialize = "k")] KeyCodeK,
    #[strum(serialize = "l")] KeyCodeL,
    #[strum(serialize = "m")] KeyCodeM,
    #[strum(serialize = "n")] KeyCodeN,
    #[strum(serialize = "o")] KeyCodeO,
    #[strum(serialize = "p")] KeyCodeP,
    #[strum(serialize = "q")] KeyCodeQ,
    #[strum(serialize = "r")] KeyCodeR,
    #[strum(serialize = "s")] KeyCodeS,
    #[strum(serialize = "t")] KeyCodeT,
    #[strum(serialize = "u")] KeyCodeU,
    #[strum(serialize = "v")] KeyCodeV,
    #[strum(serialize = "w")] KeyCodeW,
    #[strum(serialize = "x")] KeyCodeX,
    #[strum(serialize = "y")] KeyCodeY,
    #[strum(serialize = "z")] KeyCodeZ,

    // Numbers (0–9)
    #[strum(serialize = "0")] KeyCode0,
    #[strum(serialize = "1")] KeyCode1,
    #[strum(serialize = "2")] KeyCode2,
    #[strum(serialize = "3")] KeyCode3,
    #[strum(serialize = "4")] KeyCode4,
    #[strum(serialize = "5")] KeyCode5,
    #[strum(serialize = "6")] KeyCode6,
    #[strum(serialize = "7")] KeyCode7,
    #[strum(serialize = "8")] KeyCode8,
    #[strum(serialize = "9")] KeyCode9,

    // Arrow keys
    #[strum(serialize = "ArrowUp")] KeyCodeArrowUp,
    #[strum(serialize = "ArrowDown")] KeyCodeArrowDown,
    #[strum(serialize = "ArrowLeft")] KeyCodeArrowLeft,
    #[strum(serialize = "ArrowRight")] KeyCodeArrowRight,

    // misc.
    #[strum(serialize = " ")] KeyCodeSpace,
    Unknown,
}