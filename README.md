available models:
```
pub enum Model {
    #[default]
    Gemini1_0Pro,
    #[cfg(feature = "beta")]
    #[cfg_attr(docsrs, doc(cfg(feature = "beta")))]
    Gemini1_5Pro,
    #[cfg(feature = "beta")]
    #[cfg_attr(docsrs, doc(cfg(feature = "beta")))]
    Gemini1_5Flash,
    #[cfg(feature = "beta")]
    #[cfg_attr(docsrs, doc(cfg(feature = "beta")))]
    Gemini1_5Flash8B,
    #[cfg(feature = "beta")]
    #[cfg_attr(docsrs, doc(cfg(feature = "beta")))]
    Gemini2_0Flash,
    #[cfg(feature = "beta")]
    #[cfg_attr(docsrs, doc(cfg(feature = "beta")))]
    Custom(String),
    // TODO: Embedding004
}```