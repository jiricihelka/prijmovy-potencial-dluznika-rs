//! Types and constants for the API
//!
//! This module contains only definitions of types directly used by the API.
//! Structs provided in this module do not do any data transformation apart from
//! renaming fields to match rust naming conventions.
//!
//! The API also has some questionable design choices when it comes to data types. These
//! are preserved in this module, so any potential user can fix them in their own way.

use serde::{Deserialize, Serialize};

/// A type alias for monetary values
pub type Money = f64;

/// A type alias for fractions of 100
pub type Percent = f64;

/// A type alias for counts of items
pub type Count = u32;

/// The URL of the API endpoint.
pub const URL: &str = "https://msp.gov.cz/web/insolvence/nastroje/prijmovy-potencial-dluznika";

/// The raw request body sent to the API. 
/// 
/// Optional fields are represented as empty strings.
/// 
/// This `struct` directly corresponds to the form fields on the [website](https://msp.gov.cz/web/insolvence/nastroje/prijmovy-potencial-dluznika).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RawRequest {
    /// Aktuální hrubé příjmy dlužníka (v celých Kč)
    #[serde(rename = "aktualniPrijmy")]
    pub aktualni_prijmy: String,

    /// Nejvyšší příjmy dlužníka (v celých Kč)
    ///
    /// Uveďte nejvyšší hrubé měsíční příjmy, kterých dlužník dosahoval v relevantním období
    /// (12 příp. 36 měsíců) před zahájením insolvenčního řízení, resp. před schálením oddlužení.
    #[serde(rename = "nejvyssiPrijmny")]
    pub nejvyssi_prijmny: String,

    /// Obor činnosti dle CZ-NACE
    ///
    /// Klasifikace ekonomických činností (CZ-NACE) je k dispozici na [portálu ČSU](https://csu.gov.cz/klasifikace_ekonomickych_cinnosti_cz_nace).
    ///
    /// Výběr oborů je omezem dle dostupnosti statistických dat.
    pub obor: String,

    /// Požadovaný percentil příjmů
    ///
    /// Individuální schopnosti dlužníka a jeho specifické možnosti dosahovat příjmů na trhu práce lze
    /// zohlednit volbou percentilu, který poskytuje srovnání s obdobnými osobami seřazenými podle výše výdělků.
    #[serde(rename = "percentilPrijmu")]
    pub percentil_prijmu: String,

    /// Počet dětí, na které je uplatňován daňový bonus
    #[serde(rename = "pocetDeti")]
    pub pocet_deti: String,

    /// Počet dětí z `pocet_deti` s průkazem ZTP/P
    #[serde(rename = "pocetZtp")]
    pub pocet_ztp: String,

    /// Pohlaví
    pub pohlavi: String,

    /// Omezená pracovní schopnost
    pub postizeni: String,

    /// Region
    ///
    /// Kategorie regionů dle [portálu IROP](https://irop.gov.cz/cs/irop-2021-2027/kategorie-regionu)
    pub region: String,

    /// Sleva na pojistném pro pracující důchodce
    #[serde(rename = "slevaDuchodce")]
    pub sleva_duchodce: bool,

    /// Sleva na manžela/manželku
    #[serde(rename = "slevaManzelka")]
    pub sleva_manzelka: String,

    /// Sleva na poplatníka
    #[serde(rename = "slevaPoplatnik")]
    pub sleva_poplatnik: String,

    /// Datová sada
    ///
    /// Zohlední se statistická data o přijmech dosahovaných ve zvoleném období.
    #[serde(rename = "statistickeDataNazev")]
    pub statisticka_data_nazev: String,

    /// Úvazek
    pub uvazek: String,

    /// Věková kategorie
    pub vek: String,

    /// Výpočtová sada
    ///
    /// Zohlední se nastavení pro výpočet čisté mzdy, které bylo platné ve zvoleném období.
    #[serde(rename = "vypoctoveDataNazev")]
    pub vypoctova_data_nazev: String,

    /// Vzdělání
    ///
    /// Vychází se z Klasifikace kmenových oborů vzdělání (KKOV), viz [portál ČSU](https://csu.gov.cz/klasifikace_kmenovych_oboru_vzdelani_-kkov-).
    pub vzdelani: String,
}

/// The raw response body received from the API.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RawResponse {
    #[serde(rename = "rokVydaniDat")]
    rok_vydani_dat: String,

    #[serde(rename = "rokPlatnostiVypoctu")]
    rok_platnosti_vypoctu: String,

    #[serde(rename = "verzeAplikace")]
    verze_aplikace: String,

    #[serde(rename = "pocetOsobSCharakteristikami")]
    pocet_osob_s_charakteristikami: Count,

    #[serde(rename = "pozadovanaVysePrijmu")]
    pozadovana_vyse_prijmu: Money,

    #[serde(rename = "pozadovanyVysePrijmuProcenta")]
    pozadovany_vyse_prijmu_procenta: Percent,

    #[serde(rename = "aktualniPrijmy")]
    aktualni_prijmy: Money,

    #[serde(rename = "nejvyssiPrijmny")]
    nejvyssi_prijmny: Money,

    #[serde(rename = "distribucePrijmu")]
    distribuce_prijmu: Vec<DistribucePrijmuDatapoint>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DistribucePrijmuDatapoint 
{
    percentil: Percent,
    
    #[serde(rename = "hrubePrijmy")]
    hrube_prijmy: Money,

    #[serde(rename = "cistePrijmy")]
    ciste_prijmy: Money,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VypocetCistehoPrijmu {
    #[serde(rename = "zakladDane")]
    zaklad_dane: Money,

    #[serde(rename = "socialniPojisteni")]
    socialni_pojisteni: Money,

    #[serde(rename = "zdravotniPojisteni")]
    zdravotni_pojisteni: Money,
    
    #[serde(rename = "slevaNaPoplatnika")]
    sleva_na_poplatnika: Money,

    #[serde(rename = "danovyBonusNaDeti")]
    danovy_bonus_na_deti: Money,

    postizeni: Money,

    #[serde(rename = "slevaNaManzela")]
    sleva_na_manzela: Money,

    #[serde(rename = "slevaNaDuchodce")]
    sleva_na_duchodce: Money,

    #[serde(rename = "cistyPrijem")]
    cisty_prijem: Money,

    #[serde(rename = "danZeMzdyProcent")]
    dan_ze_mzdy_procent: Percent,

    #[serde(rename = "slevaNaPoplatnikaStatic")]
    sleva_na_poplatnika_static: Money,

    #[serde(rename = "duchodovePojisteniProcent")]
    duchodove_pojisteni_procent: Percent,

    #[serde(rename = "nemocenskePojisteniProcent")]
    nemocenske_pojisteni_procent: Percent,

    #[serde(rename = "zdravotniPojisteniProcent")]
    zdravotni_pojisteni_procent: Percent,

    #[serde(rename = "danFO")]
    dan_fo: Money,
}