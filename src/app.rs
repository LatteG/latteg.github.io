use spell_card::{Area, CastTime, Defence, Heightened, RollResult, SpellCard, SpellOverview, SpellType};
use yew::prelude::*;
use yew_hooks::{use_local_storage, UseLocalStorageHandle};

mod spell_card;

#[function_component(App)]
pub fn app() -> Html {
    let cards: Vec<SpellCard> = vec![
        SpellCard { 
            spell_name: "Lightningbolt".to_string(),
            cast_time: CastTime::Reaction,
            spell_type: SpellType::Cantrip,
            spell_level: 1,
            traits: format_string_vec(vec!["Lightning"]),
            overview: vec![SpellOverview::Range(30),
                        SpellOverview::Targets("1 or 2 creatures".to_string()),
                        SpellOverview::Defence(Defence::Fortitude)],
            spell_effect: "Electric arcs jump between you and the target(s).".to_string(),
            roll_effect: vec![],
            heightened: vec![Heightened::Repeat(2, "Increase damage by 1d4".to_string())]
        },
        SpellCard {
            spell_name: "Fireball".to_string(),
            cast_time: CastTime::Triple,
            spell_type: SpellType::Spell,
            spell_level: 3,
            traits: format_string_vec(vec!["Fire", "AoE"]),
            overview: vec![SpellOverview::Range(20),
                           SpellOverview::Area(Area::Burst(15))],
            spell_effect: "Cast a fireball\nTry to avoid your friends or they might want to try to kill you until you have died four times\nEach creature in the affected area makes a Reflex save".to_string(),
            roll_effect: vec![RollResult::CriticalSuccess("The creature is unaffected".to_string()),
                              RollResult::Success("The creature takes half damage".to_string()),
                              RollResult::CriticalFailure("The creature takes double damage and 3d6 persistent fire damage".to_string())],
            heightened: vec![Heightened::Repeat(1, "Increase damage by 1d6".to_string()),
                             Heightened::Single(5, "Increase persistent damage by 1d6".to_string())]
        }
    ];
    // let spell_card: SpellCard = cards[0].clone();
    // let storage: UseLocalStorageHandle<String> = use_local_storage::<String>(spell_card.spell_name.clone());
    // storage.set(serde_json::to_string(&spell_card).unwrap());
    html! {
        <main>
            <h1>{ "My PF2e Spellbook" }</h1>
            {cards.iter().map(|card| {
                card.to_html()
            }).collect::<Html>()}
        </main>
    }
}


fn format_string_vec(input:Vec<&str>) -> Vec<String> {
    input.iter().map(|s| s.to_string()).collect()
}
