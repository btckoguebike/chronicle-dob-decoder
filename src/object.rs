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
    coordinate,
    area,
    color,
    commodity,
});

declare_object!(Date {
    era,
    year,
    time,
    weather,
    holiday,
    season,
    background,
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
    use crate::generated::{character, date, location, story};
    use crate::object::{Character, Date, Location, Story};

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
    fn test_render_date() {
        set_decoder_language(Language::CN).expect("set language");

        let date = Date {
            era: decode_segment(date::ERA).unwrap(),
            year: decode_segment(date::YEAR).unwrap(),
            time: decode_segment(date::TIME).unwrap(),
            weather: decode_segment(date::WEATHER).unwrap(),
            holiday: decode_segment(date::HOLIDAY).unwrap(),
            season: decode_segment(date::SEASON).unwrap(),
            background: decode_segment(date::BACKGROUND).unwrap(),
            effect: decode_segment(date::EFFECT).unwrap(),
        };

        let dna = "0a257cbbf6e9ef6ef62f1fb958ac5349cc985b404f26a7ea";
        let render = date.render(dna).expect("render charactor");

        println!("{render}");
    }

    #[test]
    fn test_render_location() {
        set_decoder_language(Language::CN).expect("set language");

        let location = Location {
            adjective: decode_segment(location::ADJECTIVE).unwrap(),
            name: decode_segment(location::NAME).unwrap(),
            belonging: decode_segment(location::BELONGING).unwrap(),
            coordinate: decode_segment(location::COORDINATE).unwrap(),
            area: decode_segment(location::AREA).unwrap(),
            color: decode_segment(location::COLOR).unwrap(),
            commodity: decode_segment(location::COMMODITY).unwrap(),
        };

        let dna = "0a257cbbf6e9ef6ef62f1fb958ac5349cc985b404f26a7ea1dff13";
        let render = location.render(dna).expect("render charactor");

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
