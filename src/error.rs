#[repr(u64)]
#[cfg_attr(test, derive(Debug))]
pub enum Error {
    ParseLanguageTraitPoolError,
    ParseLanguageTemplatePoolError,
    ParseLanguageParagraphPoolError,
    ParseSegmentError,
    ParseRenderedNumberError,
    ParseRenderedNumbeArrayCountError,

    RenderVariableNumberPoolError,
    RenderPatternCountError,
    RenderAssembleSingleSelectorError,
    RenderAssembleDoubleSelectorError,
    RenderPoolEmptyError,
    RenderTemplateElementsCountError,
    RenderRecoverToObjectError,
    RenderTextTranslationError,
    RenderTemplateTranslationError,

    ExtractPatternNumberError,
    ExtractPatternTextError,
    ExtractSegmentNumberError,
    ExtractSegmentNumberArrayError,
    ExtractSegmentTextError,
    ExtractSegmentTextArrayError,
    ExtractSegmentMultipleArrayError,

    MatchFixedNumberArrayError,
    MatchFixedStringArrayError,
    MatchFixedMultipleArrayError,

    InvalidArgsLength,
    InvalidHexedDNAInArgs,
    InvalidLanguageInArgs,
    InvalidEmptyDNA,
    InvalidCombination,
}
