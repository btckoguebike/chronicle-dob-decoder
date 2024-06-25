#![allow(clippy::all)]
#[derive(Clone, Debug)]
pub enum Error {
    DecodeVariableNumberPoolError = 100,
    DecodePatternCountError,
    DecodeAssembleSingleSelectorError,
    DecodeAssembleDoubleSelectorError,
    DecodePoolEmptyError,
    DecodeTemplateElementsCountError,
    DecodeInsufficientDNA,

    ParsePlayerAdjectiveError,
    ParsePlayerNameError,
    ParsePlayerProfessionError,
    ParsePlayerPowerError,
    ParsePlayerGoldError,
    ParsePlayerCardError,

    ParseSceneAttributeError,
    ParseSceneNameError,
    ParseSceneOperationError,
    ParseSceneScoreError,
    ParseSceneDifficultyError,
    ParseSceneCommodityError,

    ParseEnvironmentEraError,
    ParseEnvironmentAjectiveError,
    ParseEnvironmentTimeError,
    ParseDateTemperatureError,
    ParseEnvironmentRankError,
    ParseEnvironmentModeError,
    ParseDateBackgroundError,
    ParseEnvironmentEffectError,

    ParseEventPlayerError,
    ParseEventSceneError,
    ParseEventContextError,
    ParseEventTransitionError,
    ParseEventClimaxError,
    ParseEventEndingError,

    InvalidArgsLength,
    InvalidHexedDNAInArgs,
    InvalidLanguageInArgs,
    InvalidEmptyDNA,
    InvalidCombination,
}
