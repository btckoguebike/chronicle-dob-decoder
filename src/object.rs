use serde::Serialize;

use crate::core::decoder::Segment;
use crate::core::render::{segment_render, Render};

macro_rules! declare_object {
    (
        $name:ident {
            $($var:ident,)+
        }
    ) => {
        #[derive(Serialize)]
        pub struct $name {
            $(
                #[serde(serialize_with = "segment_render")]
                pub $var: Segment,
            )+
        }

        impl Render for $name {}
    };
}

declare_object!(Character {
    adjective,
    name,
    profession,
    hp,
    power,
    attack,
    defense,
    gold,
    card,
});

declare_object!(Location {
    adjective,
    name,
    belonging,
    coordination,
    area,
    color,
    commodity,
});

declare_object!(Date {
    era,
    year,
    time,
    weather,
    emotion,
    status,
    enironment,
    activity,
    effect,
});

declare_object!(Story {
    character,
    location,
    date,
    story,
});

#[cfg(test)]
mod test {
    use crate::core::decoder::{decode_segment, set_decoder_language, Language};
    use crate::core::render::Render;
    use crate::generated::{character, story};
    use crate::object::{Character, Story};

    #[test]
    fn test_render_character() {
        set_decoder_language(Language::CN).expect("set language");

        let charactor = Character {
            adjective: decode_segment(character::ADJECTIVE).unwrap(),
            name: decode_segment(character::NAME).unwrap(),
            profession: decode_segment(character::PROFESSION).unwrap(),
            hp: decode_segment(character::HP).unwrap(),
            power: decode_segment(character::POWER).unwrap(),
            attack: decode_segment(character::ATTACK).unwrap(),
            defense: decode_segment(character::DEFENSE).unwrap(),
            gold: decode_segment(character::GOLD).unwrap(),
            card: decode_segment(character::CARD).unwrap(),
        };

        let dna = "0a257cbbf6e9ef6ef62f1fb958ac5349cc985b404f26a7ea";
        let render = charactor.render(dna).expect("render charactor");

        println!("{render}");
    }

    #[test]
    fn test_render_story() {
        set_decoder_language(Language::CN).expect("set language");

        let story = Story {
            character: decode_segment(story::CHARACTER).unwrap(),
            location: decode_segment(story::LOCATION).unwrap(),
            date: decode_segment(story::DATE).unwrap(),
            story: decode_segment(story::STORY).unwrap(),
        };

        let dna = "0a257cbbf6e9ef6ef62f1fb958ac5349cc985b404f26a7ea";
        let render = story.render(dna).expect("render story");

        println!("{render}");
    }
}
