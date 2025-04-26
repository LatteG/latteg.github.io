use std::fmt::Display;
use serde::{Deserialize, Serialize};

use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum CastTime {
    Free,
    Reaction,
    Single,
    Double,
    Triple,
    Longer(String),
}

impl Display for CastTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CastTime::Free     => write!(f, "F"),
            CastTime::Reaction => write!(f, "R"),
            CastTime::Single   => write!(f, "A"),
            CastTime::Double   => write!(f, "D"),
            CastTime::Triple   => write!(f, "T"),
            CastTime::Longer(duration) => write!(f, "{}", duration),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum SpellType {
    Cantrip,
    Focus,
    Spell,
    Innate,
    Ritual,
}

impl Display for SpellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_str: &str = match self {
            SpellType::Cantrip => "Cantrip",
            SpellType::Focus   => "Focus",
            SpellType::Spell   => "Spell",
            SpellType::Innate  => "Innate",
            SpellType::Ritual  => "Ritual",
        };
        write!(f, "{}", display_str)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub enum SpellOverview {
    Range(u8),
    Area(Area),
    Targets(String),
    Defence(Defence),
    Duration(String),
}

impl Display for SpellOverview {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let overview_str: &str = match self {
            SpellOverview::Range(_)    => "Range",
            SpellOverview::Area(_)     => "Area",
            SpellOverview::Targets(_)  => "Targets",
            SpellOverview::Defence(_)  => "Defence",
            SpellOverview::Duration(_) => "Duration",
        };
        write!(f, "{}", overview_str)
    }
}

impl SpellOverview {
    pub fn to_html(&self) -> Html {
        html!{<>
            <b>{self.to_string()}{": "}</b>{match self {
                SpellOverview::Range(range)       => format!("{}ft",range),
                SpellOverview::Area(area)         => area.to_string(),
                SpellOverview::Targets(targets)   => targets.clone(),
                SpellOverview::Defence(defence)   => defence.to_string(),
                SpellOverview::Duration(duration) => duration.clone(),
            }}
        </>}
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub enum Area {
    Burst(u8),
    Cone(u8),
    Emanation(u8),
    Line(u8, Option<u8>),
}

impl Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Area::Burst(aoe)                     => write!(f, "{}ft burst", aoe),
            Area::Cone(aoe)                      => write!(f, "{}ft cone", aoe),
            Area::Emanation(aoe)                 => write!(f, "{}ft emanation", aoe),
            Area::Line(length, None)             => write!(f, "{}ft line", length),
            Area::Line(length, Some(width)) => write!(f, "{}ft long and {}ft wide line", length, width),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub enum Defence {
    ArmourClass,
    Fortitude,
    Reflex,
    Will,
}

impl Display for Defence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Defence::ArmourClass => write!(f, "AC"),
            Defence::Fortitude   => write!(f, "Fortitude"),
            Defence::Reflex      => write!(f, "Reflex"),
            Defence::Will        => write!(f, "Will"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub enum RollResult {
    CriticalSuccess(String),
    Success(String),
    Failure(String),
    CriticalFailure(String),
}

impl Display for RollResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result_str: &str = match self {
            RollResult::CriticalSuccess(_) => "Critical Success",
            RollResult::Success(_)         => "Success",
            RollResult::Failure(_)         => "Failure",
            RollResult::CriticalFailure(_) => "Critical Failure",
        };
        write!(f, "{}", result_str)
    }
}

impl RollResult {
    pub fn to_html(&self) -> Html {
        html!{<><b>{self.to_string()}{": "}</b>{match self {
            RollResult::CriticalSuccess(eff) |
            RollResult::Success(eff) |
            RollResult::Failure(eff) |
            RollResult::CriticalFailure(eff) => eff,
        }}</>}
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub enum Heightened {
    Repeat(u8, String),
    Single(u8, String),
}

impl Display for Heightened {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Heightened::Repeat(lvl, _) => write!(f, "Heightened (+{})", lvl),
            Heightened::Single(lvl, _) => write!(f, "Heightened ({})", match lvl {
                1 => "1st".to_string(),
                2 => "2nd".to_string(),
                3 => "3rd".to_string(),
                n => format!("{}th", n)
            }),
        }
    }
}

impl Heightened {
    pub fn to_html(&self) -> Html {
        html!{<><b>{self.to_string()}{": "}</b>{match self {
            Heightened::Repeat(_, eff) | Heightened::Single(_, eff) => eff,
        }}</>}
    }
}

#[derive(PartialEq, Properties, Clone, Serialize, Deserialize)]
pub struct SpellCard {
    // Header
    pub spell_name: String,
    pub cast_time: CastTime,
    pub spell_type: SpellType,
    pub spell_level: u8,
    pub link: String,
    // Middle
    pub traits: Vec<String>,
    pub overview: Vec<SpellOverview>,
    // Bottom
    pub spell_effect: String,
    pub roll_effect: Vec<RollResult>,
    pub heightened: Vec<Heightened>
}

impl SpellCard {
    pub fn to_html(&self) -> Html {
        html!{<SpellCardHtml
            spell_name={self.spell_name.clone()}
            cast_time={self.cast_time.clone()}
            spell_type={self.spell_type.clone()}
            spell_level={self.spell_level.clone()}
            link={self.link.clone()}
            traits={self.traits.clone()}
            overview={self.overview.clone()}
            spell_effect={self.spell_effect.clone()}
            roll_effect={self.roll_effect.clone()}
            heightened={self.heightened.clone()}
        />}
    }
}

#[function_component]
pub fn SpellCardHtml(props: &SpellCard) -> Html {
    let SpellCard {
        // Header
        spell_name,
        cast_time,
        spell_type,
        spell_level,
        link,
        // Middle
        traits,
        overview,
        // Bottom
        spell_effect,
        roll_effect,
        heightened
    } = props;

    let spell_effect_paragraphs: Vec<&str> = spell_effect.split("\n").collect();

    html! {
        <div class="spellcard">
            // # Header
            <SpellHeader
                spell_name={spell_name.clone()}
                cast_time={cast_time.clone()}
                spell_type={spell_type.clone()}
                spell_level={spell_level}
                link={link.clone()}
            />
            <hr/>
            // # Middle
            // Traits
            if traits.len() > 0 {
                <div class="trait-container">
                    { traits.iter().map(|val| {
                        html!{<SpellTrait value={val.clone()} />}
                    }).collect::<Html>() }
                </div>
                <hr/>
            }
            // overview
            if overview.len() > 0 {
                { overview.clone().iter().map(|elem| {
                    html!{<div key={elem.to_string()}>{elem.to_html()}</div>}
                }).collect::<Html>() }
                <hr/>
            }
            // # Bottom
            // Effect
            { spell_effect_paragraphs.iter().map(|par| {
                html!(<div key={*par}>{par}</div>)
            }).collect::<Html>() }
            // Save/Attack results
            if roll_effect.len() > 0 {
                <hr/>
                { roll_effect.iter().map(|elem| {
                    html!{<div key={elem.to_string()}>{elem.to_html()}</div>}
                }).collect::<Html>() }
            }
            // Heightened
            if heightened.len() > 0 {
                <hr/>
                { heightened.iter().map(|elem| {
                    html!{<div key={elem.to_string()}>{elem.to_html()}</div>}
                }).collect::<Html>() }
            }
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct SpellTraitProps {
    pub value: String,
}

#[function_component]
pub fn SpellTrait(props: &SpellTraitProps) -> Html {
    let SpellTraitProps {
        value
    } = props;
    html! {
        <div class="spell-trait">
            {value}
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct SpellHeaderProps {
    // Left part
    pub spell_name: String,
    pub cast_time: CastTime,
    pub link: String,

    // Right part
    pub spell_type: SpellType,
    pub spell_level: u8,
}

#[function_component]
pub fn SpellHeader(props: &SpellHeaderProps) -> Html {
    let SpellHeaderProps {
        // Left part
        spell_name,
        cast_time,
        link,

        // Right part
        spell_type,
        spell_level
    } = props;

    html! {
        <div class="spell-header">
            <div>
                <a href={link.clone()} target="blank">{spell_name}</a>{" "}if let CastTime::Longer(cast_time) = cast_time {
                    {cast_time}
                } else {
                    <span class="action">{cast_time.to_string()}</span>
                }
            </div>
            <div>
                {spell_type.to_string()}{" "}{spell_level}
            </div>
        </div>
    }
}
