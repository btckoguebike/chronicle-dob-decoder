macro_rules! casting_error {
    (
        pub enum $name:ident {
            $($error:ident ,)+
        }
    ) => {
        #[repr(u64)]
        #[derive(Debug)]
        pub enum $name {
            $($error ,)+
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
        ParseLanguageTraitPoolError,
        ParseLanguageTemplatePoolError,
        ParseSegmentError,

        RenderVariableNumberPoolError,
        RenderPatternCountError,
        RenderAssembleSingleSelectorError,
        RenderAssembleDoubleSelectorError,
        RenderPoolEmptyError,
        RenderTemplateElementsCountError,
        RenderObjectError,

        InvalidHexedDNA,
        InvalidErrorString,
    }
);
