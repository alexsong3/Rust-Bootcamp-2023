//! The automated teller machine gives you cash after you swipe your card and enter your pin.
//! The atm may fail to give you cash if it is empty or you haven't swiped your card, or you have
//! entered the wrong pin.

use crate::traits::StateMachine;

/// The keys on the ATM keypad
#[derive(Debug, PartialEq, Hash, Clone)]
pub enum Key {
    One,
    Two,
    Three,
    Four,
    Enter,
}

/// Something you can do to the ATM
#[derive(Debug, Clone)]
pub enum Action {
    /// Swipe your card at the ATM. The attached value is the hash of the pin
    /// that should be keyed in on the keypad next.
    SwipeCard(u64),
    /// Press a key on the keypad
    PressKey(Key),
}

/// The various states of authentication possible with the ATM
#[derive(Debug, PartialEq, Clone)]
enum Auth {
    /// No session has begun yet. Waiting for the user to swipe their card
    Waiting,
    /// The user has swiped their card, providing the enclosed PIN hash.
    /// Waiting for the user to key in their pin
    Authenticating(u64),
    /// The user has authenticated. Waiting for them to key in the amount
    /// of cash to withdraw
    Authenticated,
}

/// The ATM. When a card is swiped, the ATM learns the correct pin's hash.
/// It waits for you to key in your pin. You can press as many numeric keys as
/// you like followed by enter. If the pin is incorrect, your card is returned
/// and the ATM automatically goes back to the main menu. If your pin is correct,
/// the ATM waits for you to key in an amount of money to withdraw. Withdraws
/// are bounded only by the cash in the machine (there is no account balance).
#[derive(Debug, PartialEq, Clone)]
pub struct Atm {
    /// How much money is in the ATM
    cash_inside: u64,
    /// The machine's authentication status.
    expected_pin_hash: Auth,
    /// All the keys that have been pressed since the last `Enter`
    keystroke_register: Vec<Key>,
}

//TODO
// Implement trait Default for Auth
// return Waiting status
impl Default for Auth {
    fn default() -> Self {
        Auth::Waiting
    }
}

//TODO
// Implement trait From  for &str
// Convert  elements in Key to &str
impl From<Key> for &str {
    fn from(_: Key) -> Self {
        todo!()
    }
}

fn key_up(pass: u64, starting_state: Atm) -> Atm {
    // println!("bbbbbbb {:?}", starting_state.keystroke_register.len());
    let mut keystroke_register_new: Vec<Key> =
        vec![Key::One, Key::Two, Key::Three, Key::Four, Key::Enter];
    let len_key = starting_state.keystroke_register.len();
    keystroke_register_new.truncate(len_key + 1);
    // println!("bbbbbbb {:?}", keystroke_register_new == vec![Key::One]);
    // println!("ccccc {:?}",  vec![Key::One]);

    let starting_state_new: Atm;
    match starting_state.expected_pin_hash {
        Auth::Authenticating(pass) => {
            (starting_state_new = Atm {
                expected_pin_hash: Auth::Authenticating(pass),
                keystroke_register: keystroke_register_new,
                ..starting_state
            })
        }
        Auth::Authenticated => {
            (starting_state_new = Atm {
                expected_pin_hash: Auth::Authenticated,
                keystroke_register: keystroke_register_new,
                ..starting_state
            })
        }
        _ => todo!(),
    }
    // let starting_state_new = Atm {
    //     expected_pin_hash: Auth::Authenticating(pass),
    //     keystroke_register: keystroke_register_new,
    //     ..starting_state
    // };
    starting_state_new
}

fn key_generic(starting_state: Atm) -> Atm {
    match starting_state.expected_pin_hash {
        Auth::Waiting => starting_state,
        Auth::Authenticating(pass) => key_up(pass, starting_state),
        Auth::Authenticated => key_up(0, starting_state),
        _ => todo!(),
    }
}

fn key_enter_auth(pass: u64, starting_state: Atm) -> Atm {
    //kiểm tra pin_hash với keystroke_register hash có bằng nhau ko => trường hợp hiển thị mong muốn
    let pass_hash = crate::traits::hash(&starting_state.keystroke_register);
    let len_key = starting_state.keystroke_register.len();
    // println!("ccccc {:?}",  pass_hash == pass);
    let starting_state_new: Atm;
    if len_key == 4 && pass_hash == pass {
        starting_state_new = Atm {
            expected_pin_hash: Auth::Authenticated,
            keystroke_register: Vec::new(),
            ..starting_state
        };
        // let starting_state_new = Atm {
        //     expected_pin_hash:  Auth::Authenticated,
        //     keystroke_register: Vec::new(),
        //     ..starting_state
        // };
        // starting_state_new
    } else {
        starting_state_new = Atm {
            expected_pin_hash: Auth::Waiting,
            keystroke_register: Vec::new(),
            ..starting_state
        };
        // starting_state_new
    }
    starting_state_new
}

fn key_enter_authenticated(starting_state: Atm) -> Atm {
    let len_key = starting_state.keystroke_register.len();
    let cash_inside_new: u64;
    if len_key > 1 {
        cash_inside_new = starting_state.cash_inside
    } else {
        cash_inside_new = starting_state.cash_inside - 1
    }
    let starting_state_new = Atm {
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
        cash_inside: cash_inside_new,
    };
    starting_state_new
}

fn key_enter(starting_state: Atm) -> Atm {
    match starting_state.expected_pin_hash {
        Auth::Waiting => starting_state,
        Auth::Authenticating(pass) => key_enter_auth(pass, starting_state),
        Auth::Authenticated => key_enter_authenticated(starting_state),
        _ => todo!(),
    }
}

fn key_single(key: Key, starting_state: Atm) -> Atm {
    let mut keystroke_register_new = starting_state.keystroke_register;
    keystroke_register_new.push(key);

    let starting_state_new = Atm {
        keystroke_register: keystroke_register_new,
        ..starting_state
    };
    starting_state_new
}

fn key_card(key: Key, starting_state: Atm) -> Atm {
    match key {
        Key::One => key_generic(starting_state),
        Key::Two => key_generic(starting_state),
        Key::Three => todo!(),
        Key::Four => key_single(key, starting_state),
        Key::Enter => key_enter(starting_state),
        _ => todo!(),
    }
    // todo!()
}

fn auth_card(pass: u64, starting_state: Atm) -> Atm {
    let starting_state_new = Atm {
        expected_pin_hash: Auth::Authenticating(pass),
        // cash_inside: 10,
        // keystroke_register: Vec::new(),
        ..starting_state
    };
    starting_state_new
}

impl StateMachine for Atm {
    // Notice that we are using the same type for the state as we are using for the machine this time.
    type State = Self;
    type Transition = Action;
    // Hint
    // Should use `default` method when auth status is Waiting status
    // Should use `from` method to convert  elements in Key to &str
    // Parse &str to integer to calculate amount
    // Use a hash function to verify the PIN both before and after the user presses the Enter key.
    fn next_state(starting_state: &Self::State, t: &Self::Transition) -> Self::State {
        // println!("bbbbbbb {:?}", starting_state);
        // println!("cccccc {:?}", t);

        match t {
            Action::PressKey(key) => key_card(key.clone(), starting_state.clone()),
            Action::SwipeCard(pass) => auth_card(*pass, starting_state.clone()),
            _ => todo!(),
        }

        // todo!("Final project")
    }
}

#[test]
fn sm_3_simple_swipe_card() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&start, &Action::SwipeCard(1234));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_swipe_card_again_part_way_through() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&start, &Action::SwipeCard(1234));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);

    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One, Key::Three],
    };
    let end = Atm::next_state(&start, &Action::SwipeCard(1234));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One, Key::Three],
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_press_key_before_card_swipe() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::One));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_enter_single_digit_of_pin() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::One));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One],
    };

    assert_eq!(end, expected);

    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One],
    };
    let end1 = Atm::next_state(&start, &Action::PressKey(Key::Two));
    let expected1 = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One, Key::Two],
    };

    assert_eq!(end1, expected1);
}

#[test]
fn sm_3_enter_wrong_pin() {
    // Create hash of pin
    let pin = vec![Key::One, Key::Two, Key::Three, Key::Four];
    let pin_hash = crate::traits::hash(&pin);

    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(pin_hash),
        keystroke_register: vec![Key::Three, Key::Three, Key::Three, Key::Three],
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_enter_correct_pin() {
    // Create hash of pin
    let pin = vec![Key::One, Key::Two, Key::Three, Key::Four];
    let pin_hash = crate::traits::hash(&pin);

    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(pin_hash),
        keystroke_register: vec![Key::One, Key::Two, Key::Three, Key::Four],
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_enter_single_digit_of_withdraw_amount() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::One));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: vec![Key::One],
    };

    assert_eq!(end, expected);

    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: vec![Key::One],
    };
    let end1 = Atm::next_state(&start, &Action::PressKey(Key::Four));
    let expected1 = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: vec![Key::One, Key::Four],
    };

    assert_eq!(end1, expected1);
}

#[test]
fn sm_3_try_to_withdraw_too_much() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: vec![Key::One, Key::Four],
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_withdraw_acceptable_amount() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: vec![Key::One],
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
    let expected = Atm {
        cash_inside: 9,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}
