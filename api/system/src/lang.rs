pub use sys::ffi::PDLanguage;

pub trait PDLanguageExt {
    #![allow(non_upper_case_globals)]
    const English: PDLanguage = PDLanguage::kPDLanguageEnglish;
    const Japanese: PDLanguage = PDLanguage::kPDLanguageJapanese;
    // SDK 3.1.0 renamed `kPDLanguageUnknown` to `kPDLanguageSystem` (same
    // discriminant, 2) - keeping this constant's own name as `Unknown` so
    // nothing downstream needs to change.
    const Unknown: PDLanguage = PDLanguage::kPDLanguageSystem;
}


impl PDLanguageExt for PDLanguage {}
