const ACTIONS: [&str; 4] = ["wink", "double blink", "close your eyes", "jump"];
const REVERSE_SIGNS: u8 = 16;

pub fn actions(n: u8) -> Vec<&'static str> {
    let reverse_order = match n & REVERSE_SIGNS {
        0 => false,
        _ => true,
    };

    let bin = format!("{:b}", n);
    let mut actions = bin
        .chars()
        .rev()
        .enumerate()
        .filter_map(|(idx, c)| {
            if c == '1' && idx != 4 {
                Some(ACTIONS[idx])
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if reverse_order {
        actions.reverse();
    }

    actions
}
