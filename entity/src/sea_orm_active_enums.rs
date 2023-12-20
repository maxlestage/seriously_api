//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "dietary_preferences"
)]
pub enum DietaryPreferences {
    #[sea_orm(string_value = "BeefFree")]
    BeefFree,
    #[sea_orm(string_value = "ChickenFree")]
    ChickenFree,
    #[sea_orm(string_value = "DairyFree")]
    DairyFree,
    #[sea_orm(string_value = "EggFree")]
    EggFree,
    #[sea_orm(string_value = "FishFree")]
    FishFree,
    #[sea_orm(string_value = "GlutenFree")]
    GlutenFree,
    #[sea_orm(string_value = "LactoseFree")]
    LactoseFree,
    #[sea_orm(string_value = "NutFree")]
    NutFree,
    #[sea_orm(string_value = "Omnivore")]
    Omnivore,
    #[sea_orm(string_value = "Other")]
    Other,
    #[sea_orm(string_value = "PorkFree")]
    PorkFree,
    #[sea_orm(string_value = "SeafoodFree")]
    SeafoodFree,
    #[sea_orm(string_value = "SoyFree")]
    SoyFree,
    #[sea_orm(string_value = "Unknown")]
    Unknown,
    #[sea_orm(string_value = "Vegan")]
    Vegan,
    #[sea_orm(string_value = "Vegetarian")]
    Vegetarian,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "gender")]
pub enum Gender {
    #[sea_orm(string_value = "Female")]
    Female,
    #[sea_orm(string_value = "Intersex")]
    Intersex,
    #[sea_orm(string_value = "Male")]
    Male,
    #[sea_orm(string_value = "NonBinary")]
    NonBinary,
    #[sea_orm(string_value = "Other")]
    Other,
    #[sea_orm(string_value = "Queer")]
    Queer,
    #[sea_orm(string_value = "Transgender")]
    Transgender,
    #[sea_orm(string_value = "Unknown")]
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "role")]
pub enum Role {
    #[sea_orm(string_value = "Admin")]
    Admin,
    #[sea_orm(string_value = "Free")]
    Free,
    #[sea_orm(string_value = "Gold")]
    Gold,
    #[sea_orm(string_value = "Love")]
    Love,
    #[sea_orm(string_value = "Plus")]
    Plus,
    #[sea_orm(string_value = "Premium")]
    Premium,
    #[sea_orm(string_value = "Pro")]
    Pro,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "sexuality")]
pub enum Sexuality {
    #[sea_orm(string_value = "Aromantic")]
    Aromantic,
    #[sea_orm(string_value = "Asexual")]
    Asexual,
    #[sea_orm(string_value = "Bisexual")]
    Bisexual,
    #[sea_orm(string_value = "DemiAromantic")]
    DemiAromantic,
    #[sea_orm(string_value = "DemiAsexual")]
    DemiAsexual,
    #[sea_orm(string_value = "DemiBisexual")]
    DemiBisexual,
    #[sea_orm(string_value = "DemiGreyAsexual")]
    DemiGreyAsexual,
    #[sea_orm(string_value = "DemiHeterosexual")]
    DemiHeterosexual,
    #[sea_orm(string_value = "DemiHomosexual")]
    DemiHomosexual,
    #[sea_orm(string_value = "DemiIntersex")]
    DemiIntersex,
    #[sea_orm(string_value = "DemiNonBinary")]
    DemiNonBinary,
    #[sea_orm(string_value = "DemiPansexual")]
    DemiPansexual,
    #[sea_orm(string_value = "DemiQueer")]
    DemiQueer,
    #[sea_orm(string_value = "DemiTransgender")]
    DemiTransgender,
    #[sea_orm(string_value = "Demisexual")]
    Demisexual,
    #[sea_orm(string_value = "GreyAsexual")]
    GreyAsexual,
    #[sea_orm(string_value = "Heterosexual")]
    Heterosexual,
    #[sea_orm(string_value = "Homosexual")]
    Homosexual,
    #[sea_orm(string_value = "Intersex")]
    Intersex,
    #[sea_orm(string_value = "NonBinary")]
    NonBinary,
    #[sea_orm(string_value = "Other")]
    Other,
    #[sea_orm(string_value = "Pansexual")]
    Pansexual,
    #[sea_orm(string_value = "Queer")]
    Queer,
    #[sea_orm(string_value = "Transgender")]
    Transgender,
    #[sea_orm(string_value = "Unknown")]
    Unknown,
}