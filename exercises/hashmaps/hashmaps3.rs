// hashmaps3.rs
//
// 给出了一场足球比赛的得分列表（每行一个）。每行的格式为：
// "<队伍1名称>,<队伍2名称>,<队伍1进球>,<队伍2进球>"
// 例如：England,France,4,2（英格兰进了4个球，法国进了2个球）。
//
// 你需要建立一个得分表，包含队伍名称、队伍进球数和队伍失球数。
// 一种建立得分表的方法是使用哈希映射。解决方案已经部分写好使用哈希映射，
// 请完成它以通过测试。
//
// 让我通过测试！
//
// 执行 `rustlings hint hashmaps3` 或使用 `hint` watch 子命令获取提示。

use std::collections::HashMap;

// 一个存储队伍进球详情的结构体。
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // 队伍名称是键，其关联的结构体是值。
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        
        // 更新或插入队伍1的得分
        scores.entry(team_1_name).and_modify(|team| {
            team.goals_scored += team_1_score;
            team.goals_conceded += team_2_score;
        }).or_insert(Team {
            goals_scored: team_1_score,
            goals_conceded: team_2_score,
        });

        // 更新或插入队伍2的得分
        scores.entry(team_2_name).and_modify(|team| {
            team.goals_scored += team_2_score;
            team.goals_conceded += team_1_score;
        }).or_insert(Team {
            goals_scored: team_2_score,
            goals_conceded: team_1_score,
        });
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
