use super::spell_card::SpellCard;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone, Serialize, Deserialize)]
pub struct SpellBookProps {
    pub spells: Vec<SpellCard>
}

#[function_component]
pub fn SpellBook(props: &SpellBookProps) -> Html {
    let SpellBookProps {
        spells
    } = props;
    html! {
        <div class="spell-book">
            {spells.iter().map(|spell_card| {
                spell_card.to_html()
            }).collect::<Html>()}
        </div>
    }
}
