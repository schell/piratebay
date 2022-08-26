use colored_json::ToColoredJson;

use crate::types::Torrent;

pub fn format_results(results: Vec<Torrent>) {
    println!(
        "{}",
        serde_json::to_string(&results)
            .unwrap()
            .to_colored_json_auto()
            .unwrap()
    );
}

pub fn format_result(result: Torrent) {
    println!(
        "{}",
        serde_json::to_string(&result)
            .unwrap()
            .to_colored_json_auto()
            .unwrap()
    );
}
