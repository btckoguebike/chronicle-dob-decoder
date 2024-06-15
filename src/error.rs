#![allow(clippy::all)]
#[derive(Clone, Debug)]
pub enum Error {
    DecodeVariableNumberPoolError = 100,
    DecodePatternCountError,
    DecodeAssembleSingleSelectorError,
    DecodeAssembleDoubleSelectorError,
    DecodePoolEmptyError,
    DecodeTemplateElementsCountError,

    ParseCharacterAdjectiveError,
    ParseCharacterNameError,
    ParseCharacterProfessionError,
    ParseCharacterHpError,
    ParseCharacterPowerError,
    ParseCharacterAttackError,
    ParseCharacterDefenseError,
    ParseCharacterGoldError,
    ParseCharacterCardError,

    ParseLocationAdjectiveError,
    ParseLocationNameError,
    ParseLocationBelongingError,
    ParseLocationCoordinateError,
    ParseLocationAreaError,
    ParseLocationColorError,
    ParseLocationCommodityError,

    ParseDateEraError,
    ParseDateYearError,
    ParseDateTimeError,
    ParseDateHolidayError,
    ParseDateSeasonError,
    ParseDateWeatherError,
    ParseDateBackgroundError,
    ParseDateEffectError,

    ParseStoryCharacterError,
    ParseStoryLocationError,
    ParseStoryDateError,
    ParseStoryEventError,

    InvalidArgsLength,
    InvalidHexedDNAInArgs,
    InvalidLanguageInArgs,
    InvalidEmptyDNA,
    InvalidCombination,
}
