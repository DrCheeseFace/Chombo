// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mahc::hand::error::HandErr;

#[tauri::command]
fn adder(num1: u16, num2: u16) -> String {
    let num3 = num1 + num2;
    format!("this is you number {}", num3)
}

#[tauri::command]
fn calc(input: String) -> Vec<String> {
    use mahc::hand::Hand;
    use mahc::tile_group::TileGroup;

    let tiles: Result<Vec<TileGroup>, HandErr> = input
        .split_whitespace()
        .map(|x| x.to_string().try_into().map_err(|e| e))
        .collect();
    match tiles {
        Err(e) => return vec![format!("Error: {}", e)],
        _ => (),
    }

    let tiles = tiles.unwrap();
    let win_tile: TileGroup = "Ew".to_string().try_into().unwrap();
    let prev_tile: TileGroup = "Ew".to_string().try_into().unwrap();
    let seat_tile: TileGroup = "Ew".to_string().try_into().unwrap();

    let hand: Result<Hand, mahc::hand::error::HandErr> =
        mahc::hand::Hand::new(tiles, win_tile, seat_tile, prev_tile);
    let honba = mahc::score::HonbaCounter::default();
    if hand.is_err() {
        let err = hand.err().unwrap();
        return vec![format!("Error: {}", err)];
    }

    let score = mahc::calc::get_hand_score(
        hand.unwrap(),
        None,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        honba,
    );
    if score.is_err() {
        let err = score.err().unwrap();
        return vec![format!("Error: {}", err)];
    }
    let score = score.unwrap();

    let payment = score.payment();
    let dealer_ron = payment.dealer_ron(score.honba());
    let dealer_tsumo = payment.dealer_tsumo(score.honba());
    let non_dealer_ron = payment.non_dealer_ron(score.honba());
    let non_dealer_tsumo_to_dealer = payment.non_dealer_tsumo_to_dealer(score.honba());
    let non_deadler_tsumo_to_non_dealer = payment.non_dealer_tsumo_to_non_dealer(score.honba());
    vec![
        format!("Dealer Ron: {}", dealer_ron),
        format!("Dealer Tsumo: {}", dealer_tsumo),
        format!("Non-Dealer Ron: {}", non_dealer_ron),
        format!("Non-Dealer Tsumo to Dealer: {}", non_dealer_tsumo_to_dealer),
        format!(
            "Non-Dealer Tsumo to Non-Dealer: {}",
            non_deadler_tsumo_to_non_dealer
        ),
    ]
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![adder, calc])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
