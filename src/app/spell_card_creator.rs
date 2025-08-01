use gloo_console::log;
use regex::Regex;
use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Document, EventTarget, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};

use crate::app::spell_card::{Area, CastTime, Heightened, RollResult, SpellOverview, SpellType};

use super::spell_card::{Defence, SpellCard};

#[derive(PartialEq, Properties)]
pub struct SpellCardCreatorProps {
    // pub on_card_completion: Callback<SpellCard>,
    pub on_cancellation: Callback<MouseEvent>
}

#[function_component]
pub fn SpellCardCreator(props: &SpellCardCreatorProps) -> Html {
    let SpellCardCreatorProps {
        // on_card_completion,
        on_cancellation
    } = props;

    let state: UseStateHandle<SpellCard> = use_state(|| {
        SpellCard { 
            spell_name: String::new(),
            cast_time: CastTime::Single,
            spell_type: SpellType::Spell,
            spell_level: 1,
            link: String::new(),
            traits: Vec::new(),
            overview: Vec::new(),
            spell_effect: String::new(),
            roll_effect: Vec::new(),
            heightened: Vec::new()
        }
    });
    let state_value: SpellCard = (*state).clone();
    let card_html: Html = state.to_html();
    
    let mut heightened: Vec<Heightened> = state_value.heightened.clone();
    match heightened.last() {
        Some(Heightened::Repeat(_, txt)) | Some(Heightened::Repeat(_, txt)) if !txt.is_empty() => (),
        _ => {
            log!("Add empty heightened element");
            heightened.push(Heightened::Repeat(1, String::new()))
        },
    }
    for h in heightened.clone() {
        match h {
            Heightened::Repeat(lvl, txt) => log!("Repeat: ", lvl, ", ", txt),
            Heightened::Single(lvl, txt) => log!("Single: ", lvl, ", ", txt),
        }
    }

    // # Callback functions
    // ## Spell name
    let spell_name_change: Callback<Event> = {
        let state: UseStateHandle<SpellCard> = state.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            
            let input: Option<HtmlInputElement> = target.and_then(|t: EventTarget| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let mut card: SpellCard = (*state).clone();
                card.spell_name = input.value();
                state.set(card)
            }
        })
    };
    // ## Link
    let link_change: Callback<Event> = {
        let state: UseStateHandle<SpellCard> = state.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            
            let input: Option<HtmlInputElement> = target.and_then(|t: EventTarget| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let mut card: SpellCard = (*state).clone();
                card.link = input.value();
                state.set(card)
            }
        })
    };
    // ## Cast time
    let cast_time_change: Callback<Event> = {
        let state: UseStateHandle<SpellCard> = state.clone();
            
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();

            let input: Option<HtmlSelectElement> = target.and_then(|t: EventTarget| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(input) = input {
                let mut card: SpellCard = (*state).clone();
                card.cast_time = match input.value().as_str() {
                    "free" => CastTime::Free,
                    "reaction" => CastTime::Reaction,
                    "single" => CastTime::Single,
                    "double" => CastTime::Double,
                    "triple" => CastTime::Triple,
                    "range" => CastTime::Range(1u8, 3u8),
                    "longer" => CastTime::Longer("10 min".to_string()),
                    _ => panic!("Unexpected cast time")
                };
                state.set(card)
            }
        })
    };
    let cast_time_longer_duration_change: Callback<Event> = {
        let state: UseStateHandle<SpellCard> = state.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            
            let input: Option<HtmlInputElement> = target.and_then(|t: EventTarget| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let mut card: SpellCard = (*state).clone();
                card.cast_time = CastTime::Longer(input.value());
                state.set(card)
            }
        })
    };
    let cast_time_range_min_change: Callback<Event> = {
        let state: UseStateHandle<SpellCard> = state.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            
            let input: Option<HtmlInputElement> = target.and_then(|t: EventTarget| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let mut card: SpellCard = (*state).clone();
                if let CastTime::Range(_, max) = card.cast_time {
                    card.cast_time = CastTime::Range(input.value().parse::<u8>().unwrap(), max);
                    state.set(card)
                }
            }
        })
    };
    let cast_time_range_max_change: Callback<Event> = {
        let state: UseStateHandle<SpellCard> = state.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            
            let input: Option<HtmlInputElement> = target.and_then(|t: EventTarget| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let mut card: SpellCard = (*state).clone();
                if let CastTime::Range(min, _) = card.cast_time {
                    card.cast_time = CastTime::Range(min, input.value().parse::<u8>().unwrap());
                    state.set(card)
                }
            }
        })
    };
    // ## Spell type
    let spell_type_change: Callback<Event> = {
        let state: UseStateHandle<SpellCard> = state.clone();
            
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();

            let input: Option<HtmlSelectElement> = target.and_then(|t: EventTarget| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(input) = input {
                let mut card: SpellCard = (*state).clone();
                card.spell_type = match input.value().as_str() {
                    "cantrip" => SpellType::Cantrip,
                    "focus" => SpellType::Focus,
                    "spell" => SpellType::Spell,
                    "ritual" => SpellType::Ritual,
                    _ => panic!("Unexpected spell type")
                };
                state.set(card)
            }
        })
    };
    // ## Spell level
    let spell_level_change: Callback<Event> = {
        let state: UseStateHandle<SpellCard> = state.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            
            let input: Option<HtmlInputElement> = target.and_then(|t: EventTarget| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let mut card: SpellCard = (*state).clone();
                card.spell_level = input.value().parse::<u8>().unwrap();
                state.set(card)
            }
        })
    };
    // ## Traits
    let traits_change: Callback<InputEvent> = {
        let state: UseStateHandle<SpellCard> = state.clone();

        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            
            let input: Option<HtmlTextAreaElement> = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());
            if let Some(input) = input {
                let mut card: SpellCard = (*state).clone();
                let splitter = Regex::new(r"[\s,]+").unwrap();
                card.traits = splitter.split(input.value().as_str()).map(|s| s.to_string()).collect();
                state.set(card)
            }
        })
    };
    // ## Spell overview
    let spell_overview_range_change: Callback<Event> = {
        let state: UseStateHandle<SpellCard> = state.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            
            let input: Option<HtmlInputElement> = target.and_then(|t: EventTarget| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let mut card: SpellCard = (*state).clone();
                let mut card_overview: Vec<SpellOverview> = card.overview.clone();
                remove_overview_element("Range", &card, &mut card_overview);
                let range_val: u8 = input.value().parse::<u8>().unwrap();
                if range_val > 0 {
                    card_overview.push(SpellOverview::Range(range_val));
                }
                card_overview.sort();
                card.overview = card_overview;
                state.set(card)
            }
        })
    };
    let spell_overview_area_type_change: Callback<Event> = {
        let state: UseStateHandle<SpellCard> = state.clone();
        
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();

            let input: Option<HtmlSelectElement> = target.and_then(|t: EventTarget| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(input) = input {
                let mut card: SpellCard = (*state).clone();
                let mut card_overview: Vec<SpellOverview> = card.overview.clone();
                remove_overview_element("Area", &card, &mut card_overview);
                card_overview.push(match input.value().as_str() {
                    "burst" => SpellOverview::Area(Area::Burst(5)),
                    "cone"  => SpellOverview::Area(Area::Cone(15)),
                    "eman"  => SpellOverview::Area(Area::Emanation(5)),
                    "line"  => SpellOverview::Area(Area::Line(60, None)),
                    _ => panic!("Unexpected spell type")
                });
                card_overview.sort();
                card.overview = card_overview;
                state.set(card)
            }
        })
    };
    let spell_overview_area_value_change: Callback<Event> = {
        let state: UseStateHandle<SpellCard> = state.clone();
        
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();

            let input: Option<HtmlInputElement> = target.and_then(|t: EventTarget| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let mut card: SpellCard = (*state).clone();
                let mut card_overview: Vec<SpellOverview> = card.overview.clone();
                let area: Area = if let Some(SpellOverview::Area(old_area)) = card.get_overview_element("Area") {
                    remove_overview_element("Area", &card, &mut card_overview);
                    old_area
                } else {
                    let document: Document = window().unwrap().document().unwrap();
                    let area_selector: HtmlSelectElement = document.get_element_by_id("spell_overview_area_selector").unwrap().dyn_into::<HtmlSelectElement>().unwrap();
                    match area_selector.value().as_str() {
                        "burst" => Area::Burst(0),
                        "cone"  => Area::Cone(0),
                        "eman"  => Area::Emanation(0),
                        "line"  => Area::Line(0, None),
                        _ => panic!("Unexpected spell type")
                    }
                };
                let aoe_val: u8 = input.value().parse::<u8>().unwrap();
                if aoe_val > 0 {
                    card_overview.push(SpellOverview::Area(match area {
                        Area::Burst(_) => Area::Burst(aoe_val),
                        Area::Cone(_) => Area::Cone(aoe_val),
                        Area::Emanation(_) => Area::Emanation(aoe_val),
                        Area::Line(_, secondary_aoe_val) => Area::Line(aoe_val, secondary_aoe_val),
                    }));
                }
                card_overview.sort();
                card.overview = card_overview;
                state.set(card)
            }
        })
    };
    let spell_overview_line_secondary_value_change: Callback<Event> = {
        let state: UseStateHandle<SpellCard> = state.clone();
        
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();

            let input: Option<HtmlInputElement> = target.and_then(|t: EventTarget| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let mut card: SpellCard = (*state).clone();
                let mut card_overview: Vec<SpellOverview> = card.overview.clone();
                if let Some(SpellOverview::Area(Area::Line(line_val_0, _))) = card.get_overview_element("Area") {
                    remove_overview_element("Area", &card, &mut card_overview);
                    let line_val_1: u8 = input.value().parse::<u8>().unwrap();
                    card_overview.push(SpellOverview::Area(Area::Line(line_val_0, if line_val_1 > 0 {Some(line_val_1)} else {None})));
                }
                card_overview.sort();
                card.overview = card_overview;
                state.set(card)
            }
        })
    };
    let spell_overview_targets_change: Callback<Event> = {
        let state: UseStateHandle<SpellCard> = state.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            
            let input: Option<HtmlInputElement> = target.and_then(|t: EventTarget| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let mut card: SpellCard = (*state).clone();
                let mut card_overview: Vec<SpellOverview> = card.overview.clone();
                remove_overview_element("Targets", &card, &mut card_overview);
                let targets_str: String = input.value();
                if !targets_str.is_empty() {
                    card_overview.push(SpellOverview::Targets(targets_str));
                }
                card_overview.sort();
                card.overview = card_overview;
                state.set(card)
            }
        })
    };
    let spell_overview_defence_change: Callback<Event> = {
        let state: UseStateHandle<SpellCard> = state.clone();
        
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();

            let input: Option<HtmlSelectElement> = target.and_then(|t: EventTarget| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(input) = input {
                let mut card: SpellCard = (*state).clone();
                let mut card_overview: Vec<SpellOverview> = card.overview.clone();
                remove_overview_element("Defence", &card, &mut card_overview);
                match input.value().as_str() {
                    "none" => (),
                    "ac" => card_overview.push(SpellOverview::Defence(Defence::ArmourClass)),
                    "fort" => card_overview.push(SpellOverview::Defence(Defence::Fortitude)),
                    "refl" => card_overview.push(SpellOverview::Defence(Defence::Reflex)),
                    "will" => card_overview.push(SpellOverview::Defence(Defence::Will)),
                    _ => panic!("Invalid defence type!")
                };
                card_overview.sort();
                card.overview = card_overview;
                state.set(card)
            }
        })
    };
    let spell_overview_duration_change: Callback<Event> = {
        let state: UseStateHandle<SpellCard> = state.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            
            let input: Option<HtmlInputElement> = target.and_then(|t: EventTarget| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let mut card: SpellCard = (*state).clone();
                let mut card_overview: Vec<SpellOverview> = card.overview.clone();
                remove_overview_element("Duration", &card, &mut card_overview);
                let duration_str: String = input.value();
                if !duration_str.is_empty() {
                    card_overview.push(SpellOverview::Duration(duration_str));
                }
                card_overview.sort();
                card.overview = card_overview;
                state.set(card)
            }
        })
    };
    // ## Spell effect
    let spell_effect_change: Callback<InputEvent> = {
        let state: UseStateHandle<SpellCard> = state.clone();

        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            
            let input: Option<HtmlTextAreaElement> = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());
            if let Some(input) = input {
                let mut card: SpellCard = (*state).clone();
                card.spell_effect = input.value();
                state.set(card)
            }
        })
    };
    // ## Roll effect
    let crit_succ_change: Callback<Event> = {
        let state: UseStateHandle<SpellCard> = state.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            
            let input: Option<HtmlInputElement> = target.and_then(|t: EventTarget| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let mut card: SpellCard = (*state).clone();
                let mut roll_result: Vec<RollResult> = card.roll_effect.clone();
                roll_result = roll_result.into_iter().filter(|rr| if let RollResult::CriticalSuccess(_) = rr {false} else {true}).collect();
                let crit_succ_str: String = input.value();
                if !crit_succ_str.is_empty() {
                    roll_result.push(RollResult::CriticalSuccess(crit_succ_str));
                }
                roll_result.sort();
                card.roll_effect = roll_result;
                state.set(card)
            }
        })
    };
    let succ_change: Callback<Event> = {
        let state: UseStateHandle<SpellCard> = state.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            
            let input: Option<HtmlInputElement> = target.and_then(|t: EventTarget| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let mut card: SpellCard = (*state).clone();
                let mut roll_result: Vec<RollResult> = card.roll_effect.clone();
                roll_result = roll_result.into_iter().filter(|rr| if let RollResult::Success(_) = rr {false} else {true}).collect();
                let succ_str: String = input.value();
                if !succ_str.is_empty() {
                    roll_result.push(RollResult::Success(succ_str));
                }
                roll_result.sort();
                card.roll_effect = roll_result;
                state.set(card)
            }
        })
    };
    let fail_change: Callback<Event> = {
        let state: UseStateHandle<SpellCard> = state.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            
            let input: Option<HtmlInputElement> = target.and_then(|t: EventTarget| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let mut card: SpellCard = (*state).clone();
                let mut roll_result: Vec<RollResult> = card.roll_effect.clone();
                roll_result = roll_result.into_iter().filter(|rr| if let RollResult::Failure(_) = rr {false} else {true}).collect();
                let fail_str: String = input.value();
                if !fail_str.is_empty() {
                    roll_result.push(RollResult::Failure(fail_str));
                }
                roll_result.sort();
                card.roll_effect = roll_result;
                state.set(card)
            }
        })
    };
    let crit_fail_change: Callback<Event> = {
        let state: UseStateHandle<SpellCard> = state.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            
            let input: Option<HtmlInputElement> = target.and_then(|t: EventTarget| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let mut card: SpellCard = (*state).clone();
                let mut roll_result: Vec<RollResult> = card.roll_effect.clone();
                roll_result = roll_result.into_iter().filter(|rr| if let RollResult::CriticalFailure(_) = rr {false} else {true}).collect();
                let crit_fail_str: String = input.value();
                if !crit_fail_str.is_empty() {
                    roll_result.push(RollResult::CriticalFailure(crit_fail_str));
                }
                roll_result.sort();
                card.roll_effect = roll_result;
                state.set(card)
            }
        })
    };
    // ## Heightened effect
    let heightened_text_change = |heightened_element: &Heightened| {
        let state: UseStateHandle<SpellCard> = state.clone();
        let heightened_element_clone: Heightened = heightened_element.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
                
            let input: Option<HtmlInputElement> = target.and_then(|t: EventTarget| t.dyn_into::<HtmlInputElement>().ok());
                
            if let Some(input) = input {
                let mut card: SpellCard = (*state).clone();
                let mut heightened: Vec<Heightened> = card.heightened.clone();
                let heightened_str: String = input.value();
                let new_heightened_element: Heightened = match heightened_element_clone {
                    Heightened::Repeat(lvl, _) => Heightened::Repeat(lvl, heightened_str.clone()),
                    Heightened::Single(lvl, _) => Heightened::Single(lvl, heightened_str.clone()),
                };
                card.heightened = if heightened.contains(&heightened_element_clone) {
                    heightened.into_iter().map(|h| if h == heightened_element_clone {new_heightened_element.clone()} else {h}).collect()
                } else {
                    heightened.push(new_heightened_element);
                    heightened.push(Heightened::Repeat(1, String::new()));
                    heightened
                };
                state.set(card)
            }
        })
    };
    let remove_heightened = |heightened_element: &Heightened| {
        let state: UseStateHandle<SpellCard> = state.clone();
        let heightened_element_clone: Heightened = heightened_element.clone();

        Callback::from(move |_| {
            let mut card: SpellCard = (*state).clone();
            let heightened: Vec<Heightened> = card.heightened.clone();
            card.heightened = heightened.into_iter().filter(|h| *h != heightened_element_clone).collect();
            state.set(card)
        })
    };

    html! {
        <div>
            <p>{"Tada!"}</p>
            <form>
                <label>
                    {"Spell name:"}
                    <input 
                        onchange={spell_name_change}
                        type="text"
                        value={state_value.spell_name.clone()}
                    />
                </label>
                <br/>
                <label>
                    {"Link:"}
                    <input 
                        onchange={link_change}
                        type="url"
                        value={state_value.link.clone()}
                    />
                </label>
                <br/>
                <label>
                    {"Cast time:"}
                    <select onchange={cast_time_change}>
                        <option value={"free"}>{"Free action"}</option>
                        <option value={"reaction"}>{"Reaction"}</option>
                        <option value={"single"} selected={true}>{"Single action"}</option>
                        <option value={"double"}>{"Two actions"}</option>
                        <option value={"triple"}>{"Three actions"}</option>
                        <option value={"range"}>{"Range"}</option>
                        <option value={"longer"}>{"Longer, specify"}</option>
                    </select>
                    if let CastTime::Longer(duration) = state_value.cast_time.clone() {
                        <input
                            onchange={cast_time_longer_duration_change}
                            type="text"
                            value={duration}
                        />
                    } else if let CastTime::Range(min, max) = state_value.cast_time.clone() {
                        <input
                            onchange={cast_time_range_min_change}
                            type="number"
                            value={min.to_string()}
                            min="1"
                            max="3"
                        />
                        <input
                            onchange={cast_time_range_max_change}
                            type="number"
                            value={max.to_string()}
                            min="1"
                            max="3"
                        />
                    }
                </label>
                <br/>
                <label>
                    {"Spell type:"}
                    <select onchange={spell_type_change}>
                        <option value={"cantrip"}>{"Cantrip"}</option>
                        <option value={"spell"} selected={true}>{"Spell"}</option>
                        <option value={"focus"}>{"Focus"}</option>
                        <option value={"ritual"}>{"Ritual"}</option>
                    </select>
                    <input
                        onchange={spell_level_change}
                        type="number"
                        value={state_value.spell_level.to_string()}
                        min="1"
                        max="10"
                    />
                </label>
                <hr/>
                <label>
                    {"Traits:"}
                    <textarea
                        oninput={traits_change}
                        value={state_value.traits.join("\n")}
                    />
                </label>
                <hr/>
                <label>
                    {"Range:"}
                    <input
                        onchange={spell_overview_range_change}
                        type="number"
                        value={
                            match state_value.get_overview_element("Range") {
                                Some(SpellOverview::Range(range)) => range.to_string(),
                                None => "0".to_string(),
                                _ => panic!("Invalid overview value!")
                            }
                        }
                        min={"0"}
                        step={"5"}
                    />
                </label>
                <br/>
                <label>
                    {"Area:"}
                    <select onchange={spell_overview_area_type_change} id={"spell_overview_area_selector"}>
                        <option value={"burst"} selected={true}>{"Burst"}</option>
                        <option value={"cone"}>{"Cone"}</option>
                        <option value={"eman"}>{"Emanation"}</option>
                        <option value={"line"}>{"Line"}</option>
                    </select>
                    <input
                        onchange={spell_overview_area_value_change}
                        type="number"
                        value={
                            match state_value.get_overview_element("Area") {
                                Some(SpellOverview::Area(area)) => area.get_aoe_val().0.to_string(),
                                None => "0".to_string(),
                                _ => panic!("Invalid overview value!")
                            }
                        }
                        min={"0"}
                        step={"5"}
                    />
                    if let Some(SpellOverview::Area(Area::Line(_, second_line_val))) = state_value.get_overview_element("Area") {
                        <input
                            onchange={spell_overview_line_secondary_value_change}
                            type="number"
                            value={
                                match second_line_val {
                                    Some(val) => val.to_string(),
                                    None => "0".to_string(),
                                }
                            }
                            min={"0"}
                            step={"5"}
                        />
                    }
                </label>
                <br/>
                <label>
                    {"Targets:"}
                    <input
                        onchange={spell_overview_targets_change}
                        type="text"
                        value={
                            match state_value.get_overview_element("Targets") {
                                Some(SpellOverview::Targets(target_string)) => target_string,
                                None => String::new(),
                                _ => panic!("Invalid overview value!")
                            }
                        }
                    />
                </label>
                <br/>
                <label>
                    {"Defence:"}
                    <select onchange={spell_overview_defence_change}>
                        <option value={"none"} selected={true}>{"None"}</option>
                        <option value={"ac"}>{"AC"}</option>
                        <option value={"fort"}>{"Fortitude"}</option>
                        <option value={"refl"}>{"Reflex"}</option>
                        <option value={"will"}>{"Will"}</option>
                    </select>
                </label>
                <br/>
                <label>
                    {"Duration:"}
                    <input
                        onchange={spell_overview_duration_change}
                        type="text"
                        value={
                            match state_value.get_overview_element("Duration") {
                                Some(SpellOverview::Duration(duration_string)) => duration_string,
                                None => String::new(),
                                _ => panic!("Invalid overview value!")
                            }
                        }
                    />
                </label>
                <hr/>
                <label>
                    {"Spell effect:"}
                    <textarea 
                        oninput={spell_effect_change}
                        value={state_value.spell_effect}
                    />
                </label>
                <hr/>
                <label>
                    {"Roll effect"}
                    <br/>
                    {"Critical success:"}
                    <input
                        onchange={crit_succ_change}
                        type="text"
                        value={
                            let roll_effect = state_value.roll_effect.to_owned();
                            let maybe_rr = roll_effect.iter().find(|rr| if let RollResult::CriticalSuccess(_) = rr {true} else {false});
                            match maybe_rr {
                                Some(RollResult::CriticalSuccess(rr)) => rr.to_owned(),
                                None => "".to_string(),
                                _ => panic!("Invalid roll effect value!")
                            }
                        }
                    />
                    <br/>
                    {"Success:"}
                    <input
                        onchange={succ_change}
                        type="text"
                        value={
                            let roll_effect = state_value.roll_effect.to_owned();
                            let maybe_rr = roll_effect.iter().find(|rr| if let RollResult::Success(_) = rr {true} else {false});
                            match maybe_rr {
                                Some(RollResult::Success(rr)) => rr.to_owned(),
                                None => "".to_string(),
                                _ => panic!("Invalid roll effect value!")
                            }
                        }
                    />
                    <br/>
                    {"Failure:"}
                    <input
                        onchange={fail_change}
                        type="text"
                        value={
                            let roll_effect = state_value.roll_effect.to_owned();
                            let maybe_rr = roll_effect.iter().find(|rr| if let RollResult::Failure(_) = rr {true} else {false});
                            match maybe_rr {
                                Some(RollResult::Failure(rr)) => rr.to_owned(),
                                None => "".to_string(),
                                _ => panic!("Invalid roll effect value!")
                            }
                        }
                    />
                    <br/>
                    {"Critical failure:"}
                    <input
                        onchange={crit_fail_change}
                        type="text"
                        value={
                            let roll_effect = state_value.roll_effect.to_owned();
                            let maybe_rr = roll_effect.iter().find(|rr| if let RollResult::CriticalFailure(_) = rr {true} else {false});
                            match maybe_rr {
                                Some(RollResult::CriticalFailure(rr)) => rr.to_owned(),
                                None => "".to_string(),
                                _ => panic!("Invalid roll effect value!")
                            }
                        }
                    />
                </label>
                <hr/>
                <label>
                    {"Heightened"}
                    { heightened.iter().map(|h| {
                        let (is_repeat, level, text) = match h {
                            Heightened::Repeat(lvl, txt) => (true, lvl, txt.to_owned()),
                            Heightened::Single(lvl, txt) => (false, lvl, txt.to_owned()),
                        };
                        let heightened_text_change_callback = heightened_text_change(h);
                        let remove_heightened_button_callback = remove_heightened(h);
                        html!{
                            <>
                                <br/>
                                <select>
                                    <option value={"repeat"} selected={is_repeat}>{"Repeat"}</option>
                                    <option value={"single"} selected={!is_repeat}>{"Once"}</option>
                                </select>
                                <input
                                    type="number"
                                    value={level.to_string()}
                                    min={"1"}
                                    max={"10"}
                                />
                                <input
                                    onchange={heightened_text_change_callback}
                                    type="text"
                                    value={text}
                                />
                                <button onclick={
                                    remove_heightened_button_callback
                                }>
                                    {"X"}
                                </button>
                            </>
                        }
                    }).collect::<Html>() }
                </label>
            </form>
            <hr/>
            {card_html}
            <button onclick={on_cancellation}>
                {"cancel"}
            </button>
        </div>
    }
}

fn remove_overview_element(overview_type:&str, card:&SpellCard, overview_vec:&mut Vec<SpellOverview>) {
    if let Some(range_obj) = card.get_overview_element(overview_type) {
        overview_vec.retain(|elem| *elem != range_obj);
    }
}
