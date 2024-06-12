macro_rules! casting_error {
    (
        pub enum $name:ident {
            $($error:ident $( = $n:expr)? ,)+
        }
    ) => {
        #[repr(u64)]
        #[derive(Debug)]
        pub enum $name {
            $($error $( = $n)? ,)+
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let error = match self {
                    $(
                        Self::$error => stringify!($error),
                    )+
                };
                f.write_str(error)
            }
        }

        impl From<alloc::string::String> for Error {
            fn from(value: alloc::string::String) -> Self {
                match value.as_str() {
                    $(
                        stringify!($error) => Self::$error,
                    )+
                    _ => Self::InvalidErrorString,
                }
            }
        }
    };
}

casting_error!(
    pub enum Error {
        ParseLanguageTraitPoolError = 100,
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
        RenderObjectError,
        RenderToObjectError,

        InvalidArgsLength,
        InvalidHexedDNAInArgs,
        InvalidLanguageInArgs,
        InvalidEmptyDNA,
        InvalidCombination,

        InvalidErrorString,
    }
);
