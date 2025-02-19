// @generated
#![cfg(feature = "icu_plurals")]
type DataStruct =
    <::icu_plurals::provider::OrdinalV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static KEYS: [&str; 12usize] = [
        "ar", "bn", "en", "es", "fil", "fr", "ja", "ru", "sr", "th", "tr", "und",
    ];
    static DATA: [&DataStruct; 12usize] =
        [&AR, &BN, &EN, &AR, &FIL, &FIL, &AR, &AR, &AR, &AR, &AR, &AR];
    KEYS.binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse())
        .ok()
        .map(|i| unsafe { *DATA.get_unchecked(i) })
}
static AR: DataStruct = include!("ar.rs.data");
static BN: DataStruct = include!("bn.rs.data");
static EN: DataStruct = include!("en.rs.data");
static FIL: DataStruct = include!("fil.rs.data");
