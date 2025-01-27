fn use_names_for_something_else(names: Vec<&str>) {
    println!("{:?}", names);
}

fn map_and_fold() {
    let names = vec!["Jane", "Jill", "Jack", "John"];

    let total_bytes = names
        .iter()
        .map(|name| name.len())
        .fold(0, |acc, len| acc + len);

    println!("{}", total_bytes);
    assert_eq!(total_bytes, 16);
    use_names_for_something_else(names);
}

fn destructuring() {
    // Destructuring
    let player_scores = [("Jack", 20), ("Jane", 23), ("Jill", 18), ("John", 19)];

    let players = player_scores
        .iter()
        .map(|&(player, _score)| player)
        .collect::<Vec<_>>();

    println!("{:?}", players);
    assert_eq!(players, ["Jack", "Jane", "Jill", "John"]); // the second collection is a Vec<&str>
}

fn sort_list_of_players_by_highest_score() {
    let mut teams = [[("Jack", 20), ("Jane", 23), ("Jill", 18), ("John", 19)], [
        ("Bill", 17),
        ("Brenda", 16),
        ("Brad", 18),
        ("Barbara", 17),
    ]];
    let teams_in_score_order = teams
        .iter_mut()
        .map(|team| {
            team.sort_by(|&a, &b| a.1.cmp(&b.1).reverse());
            team
        })
        .collect::<Vec<_>>();
    println!("Teams: {:?}", teams_in_score_order);
}

fn main() {
    // Map and Fold
    map_and_fold();

    // Destructuring
    destructuring();

    // Sorting, using iter_mut
    sort_list_of_players_by_highest_score();
}
