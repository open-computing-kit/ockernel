use crate::types::KeySym;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref KEYMAP: [KeySym; 256] = {
        let mut k: [KeySym; 256] = [KeySym::Null; 256];
        k[0x01] = KeySym::Escape;
        k[0x02] = KeySym::One;
        k[0x03] = KeySym::Two;
        k[0x04] = KeySym::Three;
        k[0x05] = KeySym::Four;
        k[0x06] = KeySym::Five;
        k[0x07] = KeySym::Six;
        k[0x08] = KeySym::Seven;
        k[0x09] = KeySym::Eight;
        k[0x0a] = KeySym::Nine;
        k[0x0b] = KeySym::Zero;
        k[0x0c] = KeySym::Minus;
        k[0x0d] = KeySym::Equal;
        k[0x0e] = KeySym::Backspace;
        k[0x0f] = KeySym::Tab;
        k[0x10] = KeySym::LowerQ;
        k[0x11] = KeySym::LowerW;
        k[0x12] = KeySym::LowerE;
        k[0x13] = KeySym::LowerR;
        k[0x14] = KeySym::LowerT;
        k[0x15] = KeySym::LowerY;
        k[0x16] = KeySym::LowerU;
        k[0x17] = KeySym::LowerI;
        k[0x18] = KeySym::LowerO;
        k[0x19] = KeySym::LowerP;
        k[0x1a] = KeySym::BracketLeft;
        k[0x1b] = KeySym::BracketRight;
        k[0x1c] = KeySym::Linefeed;
        k[0x1d] = KeySym::LeftCtrl;
        k[0x1e] = KeySym::LowerA;
        k[0x1f] = KeySym::LowerS;
        k[0x20] = KeySym::LowerD;
        k[0x21] = KeySym::LowerF;
        k[0x22] = KeySym::LowerG;
        k[0x23] = KeySym::LowerH;
        k[0x24] = KeySym::LowerJ;
        k[0x25] = KeySym::LowerK;
        k[0x26] = KeySym::LowerL;
        k[0x27] = KeySym::Semicolon;
        k[0x28] = KeySym::Apostrophe;
        k[0x29] = KeySym::Grave;
        k[0x2a] = KeySym::LeftShift;
        k[0x2b] = KeySym::Backslash;
        k[0x2c] = KeySym::LowerZ;
        k[0x2d] = KeySym::LowerX;
        k[0x2e] = KeySym::LowerC;
        k[0x2f] = KeySym::LowerV;
        k[0x30] = KeySym::LowerB;
        k[0x31] = KeySym::LowerN;
        k[0x32] = KeySym::LowerM;
        k[0x33] = KeySym::Comma;
        k[0x34] = KeySym::Period;
        k[0x35] = KeySym::Slash;
        k[0x36] = KeySym::RightShift;
        k[0x37] = KeySym::KPMultiply;
        k[0x38] = KeySym::Alt;
        k[0x39] = KeySym::Space;
        k[0x3a] = KeySym::CapsLock;
        k[0x3b] = KeySym::F1;
        k[0x3c] = KeySym::F2;
        k[0x3d] = KeySym::F3;
        k[0x3e] = KeySym::F4;
        k[0x3f] = KeySym::F5;
        k[0x40] = KeySym::F6;
        k[0x41] = KeySym::F7;
        k[0x42] = KeySym::F8;
        k[0x43] = KeySym::F9;
        k[0x44] = KeySym::F10;
        k[0x45] = KeySym::NumLock;
        k[0x46] = KeySym::ScrollLock;
        k[0x47] = KeySym::KP7;
        k[0x48] = KeySym::KP8;
        k[0x49] = KeySym::KP9;
        k[0x4a] = KeySym::KPSubtract;
        k[0x4b] = KeySym::KP4;
        k[0x4c] = KeySym::KP5;
        k[0x4d] = KeySym::KP6;
        k[0x4e] = KeySym::KPAdd;
        k[0x4f] = KeySym::KP1;
        k[0x50] = KeySym::KP2;
        k[0x51] = KeySym::KP3;
        k[0x52] = KeySym::KP0;
        k[0x53] = KeySym::KPPeriod;
        k[0x57] = KeySym::F11;
        k[0x58] = KeySym::F12;

        k[0x80 | 0x1c] = KeySym::KPEnter;
        k[0x80 | 0x1d] = KeySym::RightCtrl;
        k[0x80 | 0x35] = KeySym::KPDivide;
        k[0x80 | 0x38] = KeySym::AltGr;
        k[0x80 | 0x47] = KeySym::Home;
        k[0x80 | 0x48] = KeySym::Up;
        k[0x80 | 0x49] = KeySym::PageUp;
        k[0x80 | 0x4b] = KeySym::Left;
        k[0x80 | 0x4d] = KeySym::Right;
        k[0x80 | 0x4f] = KeySym::End;
        k[0x80 | 0x50] = KeySym::Down;
        k[0x80 | 0x51] = KeySym::PageDown;
        k[0x80 | 0x52] = KeySym::Insert;
        k[0x80 | 0x53] = KeySym::Delete;

        k
    };

    pub static ref KEYMAP_SHIFT: [KeySym; 256] = {
        let mut k: [KeySym; 256] = [KeySym::Null; 256];
        k[0x01] = KeySym::Escape;
        k[0x02] = KeySym::Exclam;
        k[0x03] = KeySym::At;
        k[0x04] = KeySym::NumberSign;
        k[0x05] = KeySym::Dollar;
        k[0x06] = KeySym::Percent;
        k[0x07] = KeySym::Circumflex;
        k[0x08] = KeySym::Ampersand;
        k[0x09] = KeySym::Asterisk;
        k[0x0a] = KeySym::ParenLeft;
        k[0x0b] = KeySym::ParenRight;
        k[0x0c] = KeySym::Underscore;
        k[0x0d] = KeySym::Plus;
        k[0x0e] = KeySym::Backspace;
        k[0x0f] = KeySym::Tab;
        k[0x10] = KeySym::UpperQ;
        k[0x11] = KeySym::UpperW;
        k[0x12] = KeySym::UpperE;
        k[0x13] = KeySym::UpperR;
        k[0x14] = KeySym::UpperT;
        k[0x15] = KeySym::UpperY;
        k[0x16] = KeySym::UpperU;
        k[0x17] = KeySym::UpperI;
        k[0x18] = KeySym::UpperO;
        k[0x19] = KeySym::UpperP;
        k[0x1a] = KeySym::BraceLeft;
        k[0x1b] = KeySym::BraceRight;
        k[0x1c] = KeySym::Linefeed;
        k[0x1d] = KeySym::LeftCtrl;
        k[0x1e] = KeySym::UpperA;
        k[0x1f] = KeySym::UpperS;
        k[0x20] = KeySym::UpperD;
        k[0x21] = KeySym::UpperF;
        k[0x22] = KeySym::UpperG;
        k[0x23] = KeySym::UpperH;
        k[0x24] = KeySym::UpperJ;
        k[0x25] = KeySym::UpperK;
        k[0x26] = KeySym::UpperL;
        k[0x27] = KeySym::Colon;
        k[0x28] = KeySym::DoubleQuote;
        k[0x29] = KeySym::Tilde;
        k[0x2a] = KeySym::LeftShift;
        k[0x2b] = KeySym::Bar;
        k[0x2c] = KeySym::UpperZ;
        k[0x2d] = KeySym::UpperX;
        k[0x2e] = KeySym::UpperC;
        k[0x2f] = KeySym::UpperV;
        k[0x30] = KeySym::UpperB;
        k[0x31] = KeySym::UpperN;
        k[0x32] = KeySym::UpperM;
        k[0x33] = KeySym::Less;
        k[0x34] = KeySym::Greater;
        k[0x35] = KeySym::Question;
        k[0x36] = KeySym::RightShift;
        k[0x37] = KeySym::KPMultiply;
        k[0x38] = KeySym::Alt;
        k[0x39] = KeySym::Space;
        k[0x3a] = KeySym::CapsLock;
        k[0x3b] = KeySym::F1;
        k[0x3c] = KeySym::F2;
        k[0x3d] = KeySym::F3;
        k[0x3e] = KeySym::F4;
        k[0x3f] = KeySym::F5;
        k[0x40] = KeySym::F6;
        k[0x41] = KeySym::F7;
        k[0x42] = KeySym::F8;
        k[0x43] = KeySym::F9;
        k[0x44] = KeySym::F10;
        k[0x45] = KeySym::NumLock;
        k[0x46] = KeySym::ScrollLock;
        k[0x47] = KeySym::KP7;
        k[0x48] = KeySym::KP8;
        k[0x49] = KeySym::KP9;
        k[0x4a] = KeySym::KPSubtract;
        k[0x4b] = KeySym::KP4;
        k[0x4c] = KeySym::KP5;
        k[0x4d] = KeySym::KP6;
        k[0x4e] = KeySym::KPAdd;
        k[0x4f] = KeySym::KP1;
        k[0x50] = KeySym::KP2;
        k[0x51] = KeySym::KP3;
        k[0x52] = KeySym::KP0;
        k[0x53] = KeySym::KPPeriod;
        k[0x57] = KeySym::F11;
        k[0x58] = KeySym::F12;

        k[0x80 | 0x1c] = KeySym::KPEnter;
        k[0x80 | 0x1d] = KeySym::RightCtrl;
        k[0x80 | 0x35] = KeySym::KPDivide;
        k[0x80 | 0x38] = KeySym::AltGr;
        k[0x80 | 0x47] = KeySym::Home;
        k[0x80 | 0x48] = KeySym::Up;
        k[0x80 | 0x49] = KeySym::PageUp;
        k[0x80 | 0x4b] = KeySym::Left;
        k[0x80 | 0x4d] = KeySym::Right;
        k[0x80 | 0x4f] = KeySym::End;
        k[0x80 | 0x50] = KeySym::Down;
        k[0x80 | 0x51] = KeySym::PageDown;
        k[0x80 | 0x52] = KeySym::Insert;
        k[0x80 | 0x53] = KeySym::Delete;

        k
    };

    pub static ref KEYMAP_CTRL: [KeySym; 256] = {
        let mut k: [KeySym; 256] = [KeySym::Null; 256];
        k[0x01] = KeySym::Escape;
        k[0x02] = KeySym::One;
        k[0x03] = KeySym::Two;
        k[0x04] = KeySym::Three;
        k[0x05] = KeySym::Four;
        k[0x06] = KeySym::Five;
        k[0x07] = KeySym::Six;
        k[0x08] = KeySym::Seven;
        k[0x09] = KeySym::Eight;
        k[0x0a] = KeySym::Nine;
        k[0x0b] = KeySym::Zero;
        k[0x0c] = KeySym::CtrlUnderscore;
        k[0x0d] = KeySym::Equal;
        k[0x0e] = KeySym::Backspace;
        k[0x0f] = KeySym::Tab;
        k[0x10] = KeySym::CtrlQ;
        k[0x11] = KeySym::CtrlW;
        k[0x12] = KeySym::CtrlE;
        k[0x13] = KeySym::CtrlR;
        k[0x14] = KeySym::CtrlT;
        k[0x15] = KeySym::CtrlY;
        k[0x16] = KeySym::CtrlU;
        k[0x17] = KeySym::Tab; // ctrl + i
        k[0x18] = KeySym::CtrlO;
        k[0x19] = KeySym::CtrlP;
        k[0x1a] = KeySym::BracketLeft;
        k[0x1b] = KeySym::CtrlBracketRight;
        k[0x1c] = KeySym::Linefeed;
        k[0x1d] = KeySym::LeftCtrl;
        k[0x1e] = KeySym::CtrlA;
        k[0x1f] = KeySym::CtrlS;
        k[0x20] = KeySym::CtrlD;
        k[0x21] = KeySym::CtrlF;
        k[0x22] = KeySym::CtrlG;
        k[0x23] = KeySym::Backspace; // ctrl + h
        k[0x24] = KeySym::Linefeed; // ctrl + j
        k[0x25] = KeySym::CtrlK;
        k[0x26] = KeySym::CtrlL;
        k[0x27] = KeySym::Semicolon;
        k[0x28] = KeySym::Apostrophe;
        k[0x29] = KeySym::Grave;
        k[0x2a] = KeySym::LeftShift;
        k[0x2b] = KeySym::CtrlBackslash;
        k[0x2c] = KeySym::CtrlZ;
        k[0x2d] = KeySym::CtrlX;
        k[0x2e] = KeySym::CtrlC;
        k[0x2f] = KeySym::CtrlV;
        k[0x30] = KeySym::CtrlB;
        k[0x31] = KeySym::CtrlN;
        k[0x32] = KeySym::CtrlM;
        k[0x33] = KeySym::Comma;
        k[0x34] = KeySym::Period;
        k[0x35] = KeySym::Slash;
        k[0x36] = KeySym::RightShift;
        k[0x37] = KeySym::KPMultiply;
        k[0x38] = KeySym::Alt;
        k[0x39] = KeySym::Space;
        k[0x3a] = KeySym::CapsLock;
        k[0x3b] = KeySym::F1;
        k[0x3c] = KeySym::F2;
        k[0x3d] = KeySym::F3;
        k[0x3e] = KeySym::F4;
        k[0x3f] = KeySym::F5;
        k[0x40] = KeySym::F6;
        k[0x41] = KeySym::F7;
        k[0x42] = KeySym::F8;
        k[0x43] = KeySym::F9;
        k[0x44] = KeySym::F10;
        k[0x45] = KeySym::NumLock;
        k[0x46] = KeySym::ScrollLock;
        k[0x47] = KeySym::KP7;
        k[0x48] = KeySym::KP8;
        k[0x49] = KeySym::KP9;
        k[0x4a] = KeySym::KPSubtract;
        k[0x4b] = KeySym::KP4;
        k[0x4c] = KeySym::KP5;
        k[0x4d] = KeySym::KP6;
        k[0x4e] = KeySym::KPAdd;
        k[0x4f] = KeySym::KP1;
        k[0x50] = KeySym::KP2;
        k[0x51] = KeySym::KP3;
        k[0x52] = KeySym::KP0;
        k[0x53] = KeySym::KPPeriod;
        k[0x57] = KeySym::F11;
        k[0x58] = KeySym::F12;

        k[0x80 | 0x1c] = KeySym::KPEnter;
        k[0x80 | 0x1d] = KeySym::RightCtrl;
        k[0x80 | 0x35] = KeySym::KPDivide;
        k[0x80 | 0x38] = KeySym::AltGr;
        k[0x80 | 0x47] = KeySym::Home;
        k[0x80 | 0x48] = KeySym::Up;
        k[0x80 | 0x49] = KeySym::PageUp;
        k[0x80 | 0x4b] = KeySym::Left;
        k[0x80 | 0x4d] = KeySym::Right;
        k[0x80 | 0x4f] = KeySym::End;
        k[0x80 | 0x50] = KeySym::Down;
        k[0x80 | 0x51] = KeySym::PageDown;
        k[0x80 | 0x52] = KeySym::Insert;
        k[0x80 | 0x53] = KeySym::Delete;

        k
    };

    pub static ref KEYMAP_META: [KeySym; 256] = {
        let mut k: [KeySym; 256] = [KeySym::Null; 256];
        k[0x01] = KeySym::MetaEscape;
        k[0x02] = KeySym::MetaOne;
        k[0x03] = KeySym::MetaTwo;
        k[0x04] = KeySym::MetaThree;
        k[0x05] = KeySym::MetaFour;
        k[0x06] = KeySym::MetaFive;
        k[0x07] = KeySym::MetaSix;
        k[0x08] = KeySym::MetaSeven;
        k[0x09] = KeySym::MetaEight;
        k[0x0a] = KeySym::MetaNine;
        k[0x0b] = KeySym::MetaZero;
        k[0x0c] = KeySym::MetaMinus;
        k[0x0d] = KeySym::MetaEqual;
        k[0x0e] = KeySym::MetaBackspace;
        k[0x0f] = KeySym::MetaTab;
        k[0x10] = KeySym::MetaQ;
        k[0x11] = KeySym::MetaW;
        k[0x12] = KeySym::MetaE;
        k[0x13] = KeySym::MetaR;
        k[0x14] = KeySym::MetaT;
        k[0x15] = KeySym::MetaY;
        k[0x16] = KeySym::MetaU;
        k[0x17] = KeySym::MetaI;
        k[0x18] = KeySym::MetaO;
        k[0x19] = KeySym::MetaP;
        k[0x1a] = KeySym::MetaBracketLeft;
        k[0x1b] = KeySym::MetaBracketRight;
        k[0x1c] = KeySym::MetaLinefeed;
        k[0x1d] = KeySym::Null;
        k[0x1e] = KeySym::MetaA;
        k[0x1f] = KeySym::MetaS;
        k[0x20] = KeySym::MetaD;
        k[0x21] = KeySym::MetaF;
        k[0x22] = KeySym::MetaG;
        k[0x23] = KeySym::MetaH;
        k[0x24] = KeySym::MetaJ;
        k[0x25] = KeySym::MetaK;
        k[0x26] = KeySym::MetaL;
        k[0x27] = KeySym::MetaSemicolon;
        k[0x28] = KeySym::MetaApostrophe;
        k[0x29] = KeySym::MetaGrave;
        k[0x2a] = KeySym::Null;
        k[0x2b] = KeySym::MetaBackslash;
        k[0x2c] = KeySym::MetaZ;
        k[0x2d] = KeySym::MetaX;
        k[0x2e] = KeySym::MetaC;
        k[0x2f] = KeySym::MetaV;
        k[0x30] = KeySym::MetaB;
        k[0x31] = KeySym::MetaN;
        k[0x32] = KeySym::MetaM;
        k[0x33] = KeySym::MetaComma;
        k[0x34] = KeySym::MetaPeriod;
        k[0x35] = KeySym::MetaSlash;
        k[0x36] = KeySym::Null;
        k[0x37] = KeySym::KPMultiply;
        k[0x38] = KeySym::Alt;
        k[0x39] = KeySym::MetaSpace;
        k[0x3a] = KeySym::CapsLock;
        k[0x3b] = KeySym::F1;
        k[0x3c] = KeySym::F2;
        k[0x3d] = KeySym::F3;
        k[0x3e] = KeySym::F4;
        k[0x3f] = KeySym::F5;
        k[0x40] = KeySym::F6;
        k[0x41] = KeySym::F7;
        k[0x42] = KeySym::F8;
        k[0x43] = KeySym::F9;
        k[0x44] = KeySym::F10;
        k[0x45] = KeySym::NumLock;
        k[0x46] = KeySym::ScrollLock;
        k[0x47] = KeySym::KP7;
        k[0x48] = KeySym::KP8;
        k[0x49] = KeySym::KP9;
        k[0x4a] = KeySym::KPSubtract;
        k[0x4b] = KeySym::KP4;
        k[0x4c] = KeySym::KP5;
        k[0x4d] = KeySym::KP6;
        k[0x4e] = KeySym::KPAdd;
        k[0x4f] = KeySym::KP1;
        k[0x50] = KeySym::KP2;
        k[0x51] = KeySym::KP3;
        k[0x52] = KeySym::KP0;
        k[0x53] = KeySym::KPPeriod;
        k[0x57] = KeySym::F11;
        k[0x58] = KeySym::F12;

        k[0x80 | 0x1c] = KeySym::KPEnter;
        k[0x80 | 0x1d] = KeySym::RightCtrl;
        k[0x80 | 0x35] = KeySym::KPDivide;
        k[0x80 | 0x38] = KeySym::AltGr;
        k[0x80 | 0x47] = KeySym::Home;
        k[0x80 | 0x48] = KeySym::Up;
        k[0x80 | 0x49] = KeySym::PageUp;
        k[0x80 | 0x4b] = KeySym::Left;
        k[0x80 | 0x4d] = KeySym::Right;
        k[0x80 | 0x4f] = KeySym::End;
        k[0x80 | 0x50] = KeySym::Down;
        k[0x80 | 0x51] = KeySym::PageDown;
        k[0x80 | 0x52] = KeySym::Insert;
        k[0x80 | 0x53] = KeySym::Delete;

        k
    };

    pub static ref KEYMAP_META_SHIFT: [KeySym; 256] = {
        let mut k: [KeySym; 256] = [KeySym::MetaNull; 256];
        k[0x01] = KeySym::MetaEscape;
        k[0x02] = KeySym::MetaExclam;
        k[0x03] = KeySym::MetaAt;
        k[0x04] = KeySym::MetaNumberSign;
        k[0x05] = KeySym::MetaDollar;
        k[0x06] = KeySym::MetaPercent;
        k[0x07] = KeySym::MetaCircumflex;
        k[0x08] = KeySym::MetaAmpersand;
        k[0x09] = KeySym::MetaAsterisk;
        k[0x0a] = KeySym::MetaParenLeft;
        k[0x0b] = KeySym::MetaParenRight;
        k[0x0c] = KeySym::MetaUnderscore;
        k[0x0d] = KeySym::MetaPlus;
        k[0x0e] = KeySym::MetaBackspace;
        k[0x0f] = KeySym::MetaTab;
        k[0x10] = KeySym::MetaShiftQ;
        k[0x11] = KeySym::MetaShiftW;
        k[0x12] = KeySym::MetaShiftE;
        k[0x13] = KeySym::MetaShiftR;
        k[0x14] = KeySym::MetaShiftT;
        k[0x15] = KeySym::MetaShiftY;
        k[0x16] = KeySym::MetaShiftU;
        k[0x17] = KeySym::MetaShiftI;
        k[0x18] = KeySym::MetaShiftO;
        k[0x19] = KeySym::MetaShiftP;
        k[0x1a] = KeySym::MetaBraceLeft;
        k[0x1b] = KeySym::MetaBraceRight;
        k[0x1c] = KeySym::MetaLinefeed;
        k[0x1d] = KeySym::Null;
        k[0x1e] = KeySym::MetaShiftA;
        k[0x1f] = KeySym::MetaShiftS;
        k[0x20] = KeySym::MetaShiftD;
        k[0x21] = KeySym::MetaShiftF;
        k[0x22] = KeySym::MetaShiftG;
        k[0x23] = KeySym::MetaShiftH;
        k[0x24] = KeySym::MetaShiftJ;
        k[0x25] = KeySym::MetaShiftK;
        k[0x26] = KeySym::MetaShiftL;
        k[0x27] = KeySym::MetaColon;
        k[0x28] = KeySym::MetaDoubleQuote;
        k[0x29] = KeySym::MetaTilde;
        k[0x2a] = KeySym::Null;
        k[0x2b] = KeySym::MetaBar;
        k[0x2c] = KeySym::MetaShiftZ;
        k[0x2d] = KeySym::MetaShiftX;
        k[0x2e] = KeySym::MetaShiftC;
        k[0x2f] = KeySym::MetaShiftV;
        k[0x30] = KeySym::MetaShiftB;
        k[0x31] = KeySym::MetaShiftN;
        k[0x32] = KeySym::MetaShiftM;
        k[0x33] = KeySym::MetaLess;
        k[0x34] = KeySym::MetaGreater;
        k[0x35] = KeySym::MetaQuestion;
        k[0x36] = KeySym::Null;
        k[0x37] = KeySym::KPMultiply;
        k[0x38] = KeySym::Alt;
        k[0x39] = KeySym::MetaSpace;
        k[0x3a] = KeySym::CapsLock;
        k[0x3b] = KeySym::F1;
        k[0x3c] = KeySym::F2;
        k[0x3d] = KeySym::F3;
        k[0x3e] = KeySym::F4;
        k[0x3f] = KeySym::F5;
        k[0x40] = KeySym::F6;
        k[0x41] = KeySym::F7;
        k[0x42] = KeySym::F8;
        k[0x43] = KeySym::F9;
        k[0x44] = KeySym::F10;
        k[0x45] = KeySym::NumLock;
        k[0x46] = KeySym::ScrollLock;
        k[0x47] = KeySym::KP7;
        k[0x48] = KeySym::KP8;
        k[0x49] = KeySym::KP9;
        k[0x4a] = KeySym::KPSubtract;
        k[0x4b] = KeySym::KP4;
        k[0x4c] = KeySym::KP5;
        k[0x4d] = KeySym::KP6;
        k[0x4e] = KeySym::KPAdd;
        k[0x4f] = KeySym::KP1;
        k[0x50] = KeySym::KP2;
        k[0x51] = KeySym::KP3;
        k[0x52] = KeySym::KP0;
        k[0x53] = KeySym::KPPeriod;
        k[0x57] = KeySym::F11;
        k[0x58] = KeySym::F12;

        k[0x80 | 0x1c] = KeySym::KPEnter;
        k[0x80 | 0x1d] = KeySym::RightCtrl;
        k[0x80 | 0x35] = KeySym::KPDivide;
        k[0x80 | 0x38] = KeySym::AltGr;
        k[0x80 | 0x47] = KeySym::Home;
        k[0x80 | 0x48] = KeySym::Up;
        k[0x80 | 0x49] = KeySym::PageUp;
        k[0x80 | 0x4b] = KeySym::Left;
        k[0x80 | 0x4d] = KeySym::Right;
        k[0x80 | 0x4f] = KeySym::End;
        k[0x80 | 0x50] = KeySym::Down;
        k[0x80 | 0x51] = KeySym::PageDown;
        k[0x80 | 0x52] = KeySym::Insert;
        k[0x80 | 0x53] = KeySym::Delete;

        k
    };

    pub static ref KEYMAP_META_CTRL: [KeySym; 256] = {
        let mut k: [KeySym; 256] = [KeySym::MetaNull; 256];
        k[0x01] = KeySym::MetaEscape;
        k[0x02] = KeySym::MetaOne;
        k[0x03] = KeySym::MetaTwo;
        k[0x04] = KeySym::MetaThree;
        k[0x05] = KeySym::MetaFour;
        k[0x06] = KeySym::MetaFive;
        k[0x07] = KeySym::MetaSix;
        k[0x08] = KeySym::MetaSeven;
        k[0x09] = KeySym::MetaEight;
        k[0x0a] = KeySym::MetaNine;
        k[0x0b] = KeySym::MetaZero;
        k[0x0c] = KeySym::MetaCtrlUnderscore;
        k[0x0d] = KeySym::MetaEqual;
        k[0x0e] = KeySym::MetaBackspace;
        k[0x0f] = KeySym::MetaTab;
        k[0x10] = KeySym::MetaCtrlQ;
        k[0x11] = KeySym::MetaCtrlW;
        k[0x12] = KeySym::MetaCtrlE;
        k[0x13] = KeySym::MetaCtrlR;
        k[0x14] = KeySym::MetaCtrlT;
        k[0x15] = KeySym::MetaCtrlY;
        k[0x16] = KeySym::MetaCtrlU;
        k[0x17] = KeySym::MetaTab; // ctrl + i
        k[0x18] = KeySym::MetaCtrlO;
        k[0x19] = KeySym::MetaCtrlP;
        k[0x1a] = KeySym::MetaBracketLeft;
        k[0x1b] = KeySym::MetaCtrlBracketRight;
        k[0x1c] = KeySym::MetaLinefeed;
        k[0x1d] = KeySym::Null;
        k[0x1e] = KeySym::MetaCtrlA;
        k[0x1f] = KeySym::MetaCtrlS;
        k[0x20] = KeySym::MetaCtrlD;
        k[0x21] = KeySym::MetaCtrlF;
        k[0x22] = KeySym::MetaCtrlG;
        k[0x23] = KeySym::MetaBackspace; // ctrl + h
        k[0x24] = KeySym::MetaLinefeed; // ctrl + j
        k[0x25] = KeySym::MetaCtrlK;
        k[0x26] = KeySym::MetaCtrlL;
        k[0x27] = KeySym::MetaSemicolon;
        k[0x28] = KeySym::MetaApostrophe;
        k[0x29] = KeySym::MetaGrave;
        k[0x2a] = KeySym::Null;
        k[0x2b] = KeySym::MetaCtrlBackslash;
        k[0x2c] = KeySym::MetaCtrlZ;
        k[0x2d] = KeySym::MetaCtrlX;
        k[0x2e] = KeySym::MetaCtrlC;
        k[0x2f] = KeySym::MetaCtrlV;
        k[0x30] = KeySym::MetaCtrlB;
        k[0x31] = KeySym::MetaCtrlN;
        k[0x32] = KeySym::MetaCtrlM;
        k[0x33] = KeySym::MetaComma;
        k[0x34] = KeySym::MetaPeriod;
        k[0x35] = KeySym::MetaSlash;
        k[0x36] = KeySym::Null;
        k[0x37] = KeySym::KPMultiply;
        k[0x38] = KeySym::Alt;
        k[0x39] = KeySym::MetaSpace;
        k[0x3a] = KeySym::CapsLock;
        k[0x3b] = KeySym::F1;
        k[0x3c] = KeySym::F2;
        k[0x3d] = KeySym::F3;
        k[0x3e] = KeySym::F4;
        k[0x3f] = KeySym::F5;
        k[0x40] = KeySym::F6;
        k[0x41] = KeySym::F7;
        k[0x42] = KeySym::F8;
        k[0x43] = KeySym::F9;
        k[0x44] = KeySym::F10;
        k[0x45] = KeySym::NumLock;
        k[0x46] = KeySym::ScrollLock;
        k[0x47] = KeySym::KP7;
        k[0x48] = KeySym::KP8;
        k[0x49] = KeySym::KP9;
        k[0x4a] = KeySym::KPSubtract;
        k[0x4b] = KeySym::KP4;
        k[0x4c] = KeySym::KP5;
        k[0x4d] = KeySym::KP6;
        k[0x4e] = KeySym::KPAdd;
        k[0x4f] = KeySym::KP1;
        k[0x50] = KeySym::KP2;
        k[0x51] = KeySym::KP3;
        k[0x52] = KeySym::KP0;
        k[0x53] = KeySym::KPPeriod;
        k[0x57] = KeySym::F11;
        k[0x58] = KeySym::F12;

        k[0x80 | 0x1c] = KeySym::KPEnter;
        k[0x80 | 0x1d] = KeySym::RightCtrl;
        k[0x80 | 0x35] = KeySym::KPDivide;
        k[0x80 | 0x38] = KeySym::AltGr;
        k[0x80 | 0x47] = KeySym::Home;
        k[0x80 | 0x48] = KeySym::Up;
        k[0x80 | 0x49] = KeySym::PageUp;
        k[0x80 | 0x4b] = KeySym::Left;
        k[0x80 | 0x4d] = KeySym::Right;
        k[0x80 | 0x4f] = KeySym::End;
        k[0x80 | 0x50] = KeySym::Down;
        k[0x80 | 0x51] = KeySym::PageDown;
        k[0x80 | 0x52] = KeySym::Insert;
        k[0x80 | 0x53] = KeySym::Delete;

        k
    };
}
