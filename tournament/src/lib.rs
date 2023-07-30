use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {
    let mut res = vec!["Team                           | MP |  W |  D |  L |  P".to_string()];
    if match_results.is_empty() {
        return res.remove(0);
    }

    let info = statistics(match_results);
    let mut info = info
        .into_iter()
        .map(|g| g.print_info(res[0].len()))
        .collect();

    res.append(&mut info);
    res.join("\n")
}

struct GameInfo<'a> {
    team_name: &'a str,
    matches_played: usize, // 比赛局数
    matches_won: usize,    // 赢的局数
    matches_drawn: usize,  // 平的局数
    matches_lost: usize,   // 输的局数
    points: usize,         // 总分, 得分规则: 赢一局得3分, 平一局得1分, 输一局得0分
}

impl<'a> GameInfo<'a> {
    fn new(team_name: &'a str) -> Self {
        GameInfo {
            team_name,
            matches_played: 0,
            matches_won: 0,
            matches_drawn: 0,
            matches_lost: 0,
            points: 0,
        }
    }
    fn win(&mut self) {
        self.matches_played += 1;
        self.matches_won += 1;
        self.points += 3;
    }
    fn loss(&mut self) {
        self.matches_played += 1;
        self.matches_lost += 1;
    }
    fn draw(&mut self) {
        self.matches_played += 1;
        self.matches_drawn += 1;
        self.points += 1;
    }

    fn print_info(&self, width: usize) -> String {
        let points = format!(
            "|  {} |  {} |  {} |  {} |  {}",
            self.matches_played,
            self.matches_won,
            self.matches_drawn,
            self.matches_lost,
            self.points
        );
        let points_len = points.len();
        let name_len = self.team_name.len();
        let space_len = width - name_len - points_len;
        self.team_name.to_string() + &" ".repeat(space_len) + &points
    }
}

fn statistics(input: &str) -> Vec<GameInfo> {
    let mut info_map = HashMap::new();

    for a_game in input.split('\n') {
        let game_info = a_game.split(';').collect::<Vec<_>>();
        let game_result = game_info[2];
        for i in 0..=1 {
            process_game_result(i, game_result, &mut info_map, game_info[i]);
        }
    }

    let mut game_info = info_map.into_values().collect::<Vec<_>>();
    // sort
    let _ = game_info
        .sort_unstable_by(|a, b| b.points.cmp(&a.points).then(a.team_name.cmp(b.team_name)));
    game_info
}

fn process_game_result<'a>(
    idx: usize,
    game_result: &str,
    info_map: &mut HashMap<&'a str, GameInfo<'a>>,
    team_name: &'a str,
) {
    let game_info = info_map
        .entry(team_name)
        .or_insert(GameInfo::new(team_name));

    match (idx, game_result) {
        (0, "win") => {
            // first team win
            game_info.win();
        }
        (1, "win") => {
            // second team loss
            game_info.loss();
        }
        (0, "loss") => {
            // first team loss
            game_info.loss();
        }
        (1, "loss") => {
            game_info.win();
        }
        _ => {
            game_info.draw();
        }
    }
}
