use sea_orm_migration::{
    prelude::*,
    sea_orm::{EnumIter, Iterable},
    sea_query::extension::postgres::Type,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Gender::Table)
                    .values(Gender::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(Sexuality::Table)
                    .values(Sexuality::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(DietaryPreferences::Table)
                    .values(DietaryPreferences::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(Role::Table)
                    .values(Role::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(User::Mail).string().not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::Firstname).string().not_null())
                    .col(ColumnDef::new(User::Lastname).string().not_null())
                    .col(ColumnDef::new(User::Age).integer().not_null())
                    .col(
                        ColumnDef::new(User::Gender)
                            .enumeration(
                                Gender::Table,
                                [
                                    Gender::Unknown,
                                    Gender::Male,
                                    Gender::Female,
                                    Gender::Transgender,
                                    Gender::NonBinary,
                                    Gender::Intersex,
                                    Gender::Queer,
                                    Gender::Other,
                                ],
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::Sexuality)
                            .enumeration(
                                Sexuality::Table,
                                [
                                    Sexuality::Unknown,
                                    Sexuality::Aromantic,
                                    Sexuality::Asexual,
                                    Sexuality::Bisexual,
                                    Sexuality::DemiAromantic,
                                    Sexuality::DemiAsexual,
                                    Sexuality::DemiBisexual,
                                    Sexuality::DemiGreyAsexual,
                                    Sexuality::DemiHeterosexual,
                                    Sexuality::DemiHomosexual,
                                    Sexuality::DemiIntersex,
                                    Sexuality::DemiNonBinary,
                                    Sexuality::DemiPansexual,
                                    Sexuality::DemiQueer,
                                    Sexuality::DemiTransgender,
                                    Sexuality::Demisexual,
                                    Sexuality::GreyAsexual,
                                    Sexuality::Heterosexual,
                                    Sexuality::Homosexual,
                                    Sexuality::Intersex,
                                    Sexuality::NonBinary,
                                    Sexuality::Other,
                                    Sexuality::Pansexual,
                                    Sexuality::Queer,
                                    Sexuality::Transgender,
                                ],
                            )
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::Biography).text())
                    .col(ColumnDef::new(User::Location).json())
                    .col(ColumnDef::new(User::Employer).string())
                    .col(ColumnDef::new(User::SchoolGrade).string())
                    .col(ColumnDef::new(User::DietaryPreferences).enumeration(
                        DietaryPreferences::Table,
                        [
                            DietaryPreferences::Unknown,
                            DietaryPreferences::BeefFree,
                            DietaryPreferences::ChickenFree,
                            DietaryPreferences::DairyFree,
                            DietaryPreferences::EggFree,
                            DietaryPreferences::FishFree,
                            DietaryPreferences::GlutenFree,
                            DietaryPreferences::LactoseFree,
                            DietaryPreferences::NutFree,
                            DietaryPreferences::Omnivore,
                            DietaryPreferences::PorkFree,
                            DietaryPreferences::Other,
                            DietaryPreferences::SeafoodFree,
                            DietaryPreferences::SoyFree,
                            DietaryPreferences::Vegan,
                            DietaryPreferences::Vegetarian,
                        ],
                    ))
                    .col(ColumnDef::new(User::CompanyRole).string())
                    .col(ColumnDef::new(User::Role).enumeration(
                        Role::Table,
                        [
                            Role::Free,
                            Role::Admin,
                            Role::Gold,
                            Role::Love,
                            Role::Plus,
                            Role::Premium,
                            Role::Pro,
                        ],
                    ))
                    .col(ColumnDef::new(User::Like).json())
                    .col(ColumnDef::new(User::LikeBy).json())
                    .col(ColumnDef::new(User::Dislike).json())
                    .col(ColumnDef::new(User::DislikeBy).json())
                    .col(ColumnDef::new(User::Gallery).json())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    CreatedAt,
    Mail,
    Password,
    Firstname,
    Lastname,
    Age,
    Gender,
    Sexuality,
    Biography,
    Location,
    Employer,
    SchoolGrade,
    DietaryPreferences,
    CompanyRole,
    Role,
    Like,
    LikeBy,
    Dislike,
    DislikeBy,
    Gallery,
}

#[derive(Iden, EnumIter)]
pub enum Gender {
    Table,
    // Male: Identifies as male.
    #[iden = "Male"]
    Male,
    // Female: Identifies as female.
    #[iden = "Female"]
    Female,
    // Transgender: Identifies as a gender different from their assigned sex at birth.
    #[iden = "Transgender"]
    Transgender,
    // NonBinary: Does not identify as exclusively male or female.
    #[iden = "NonBinary"]
    NonBinary,
    // Intersex: Possesses a combination of male and female sex characteristics.
    #[iden = "Intersex"]
    Intersex,
    // Queer: Identifies as outside of heterosexual and gender norms.
    #[iden = "Queer"]
    Queer,
    // Other: Another gender that is not listed above.
    #[iden = "Other"]
    Other,
    // Unknown: No specific gender.
    #[iden = "Unknown"]
    Unknown,
}

#[derive(Iden, EnumIter)]
pub enum Sexuality {
    Table,
    // Heterosexual: Attracted to the opposite sex.
    #[iden = "Heterosexual"]
    Heterosexual,
    // Homosexual: Attracted to the same sex.
    #[iden = "Homosexual"]
    Homosexual,
    // Bisexual: Attracted to both sexes.
    #[iden = "Bisexual"]
    Bisexual,
    // Pansexual: Attracted to all genders.
    #[iden = "Pansexual"]
    Pansexual,
    // Asexual: Does not feel sexual or romantic attraction towards others.
    #[iden = "Asexual"]
    Asexual,
    // Transgender: Identifies their gender as different from their biological sex.
    #[iden = "Transgender"]
    Transgender,
    // Non-binary: Does not identify as male or female.
    #[iden = "NonBinary"]
    NonBinary,
    // Intersex: Possesses sex characteristics that do not conform to traditional gender norms.
    #[iden = "Intersex"]
    Intersex,
    // Queer: Refers to people who identify outside of heterosexual and gender norms.
    #[iden = "Queer"]
    Queer,
    // Demisexual: Feels sexual desire mainly due to affection or emotional attachment towards a person.
    #[iden = "Demisexual"]
    Demisexual,
    // Aromantic: Does not feel romantic attraction towards others.
    #[iden = "Aromantic"]
    Aromantic,
    // Grey-Asexual: Feels little or no sexual attraction, but not necessarily none.
    #[iden = "GreyAsexual"]
    GreyAsexual,
    // Demi-Heterosexual: Attracted to both sexes, but with a preference for one.
    #[iden = "DemiHeterosexual"]
    DemiHeterosexual,
    // Demi-Homosexual: Attracted to both sexes, but with a preference for one.
    #[iden = "DemiHomosexual"]
    DemiHomosexual,
    // Demi-Bisexual: Attracted to both sexes, but with a preference for one.
    #[iden = "DemiBisexual"]
    DemiBisexual,
    // Demi-Pansexual: Attracted to all genders, but with a preference for one.
    #[iden = "DemiPansexual"]
    DemiPansexual,
    // Demi-Asexual: Feels little or no sexual attraction, but not necessarily none, with a preference for one.
    #[iden = "DemiAsexual"]
    DemiAsexual,
    // Demi-Transgender: Identifies their gender as different from their biological sex, but with a preference for one.
    #[iden = "DemiTransgender"]
    DemiTransgender,
    // Demi-Non-Binary: Does not identify as male or female, but with a preference for one.
    #[iden = "DemiNonBinary"]
    DemiNonBinary,
    // Demi-Intersex: Possesses sex characteristics that do not conform to traditional gender norms, but with a preference for one.
    #[iden = "DemiIntersex"]
    DemiIntersex,
    // Demi-Queer: Refers to people who identify outside of heterosexual and gender norms, but with a preference for one.
    #[iden = "DemiQueer"]
    DemiQueer,
    // Demi-Aromantic: Does not feel romantic attraction towards others, but with a preference for one.
    #[iden = "DemiAromantic"]
    DemiAromantic,
    // Demi-Grey-Asexual: Feels little or no sexual attraction, but not necessarily none, with a preference for one.
    #[iden = "DemiGreyAsexual"]
    DemiGreyAsexual,
    // Other: Another gender that is not listed above.
    #[iden = "Other"]
    Other,
    // Unknown: No specific sexual orientation.
    #[iden = "Unknown"]
    Unknown,
}
#[derive(Iden, EnumIter)]
pub enum DietaryPreferences {
    Table,
    // Vegetarian: A person who does not eat meat or fish.
    #[iden = "Vegetarian"]
    Vegetarian,
    // Vegan: A person who does not eat meat, fish, dairy, eggs, or any other animal product.
    #[iden = "Vegan"]
    Vegan,
    // GlutenFree: A person who does not eat gluten, a protein found in wheat, barley, and rye.
    #[iden = "GlutenFree"]
    GlutenFree,
    // DairyFree: A person who does not eat dairy, including milk, cheese, and yogurt.
    #[iden = "DairyFree"]
    DairyFree,
    // NutFree: A person who does not eat nuts or anything that contains nuts.
    #[iden = "NutFree"]
    NutFree,
    // SeafoodFree: A person who does not eat seafood.
    #[iden = "SeafoodFree"]
    SeafoodFree,
    // EggFree: A person who does not eat eggs.
    #[iden = "EggFree"]
    EggFree,
    // FishFree: A person who does not eat fish.
    #[iden = "FishFree"]
    FishFree,
    // PorkFree: A person who does not eat pork.
    #[iden = "PorkFree"]
    PorkFree,
    // ChickenFree: A person who does not eat chicken.
    #[iden = "ChickenFree"]
    ChickenFree,
    // BeefFree: A person who does not eat beef.
    #[iden = "BeefFree"]
    BeefFree,
    // LactoseFree: A person who does not eat or drink lactose.
    #[iden = "LactoseFree"]
    LactoseFree,
    // SoyFree: A person who does not eat or drink soy.
    #[iden = "SoyFree"]
    SoyFree,
    // Omnivore: A person who eats all kinds of food.
    #[iden = "Omnivore"]
    Omnivore,
    // Other: Another dietary preference that is not listed above.
    #[iden = "Other"]
    Other,
    // Unknown: No specific dietary preference.
    #[iden = "Unknown"]
    Unknown,
}

#[derive(Iden, EnumIter)]
pub enum Role {
    Table,
    // Free: The role of a user.
    #[iden = "Free"]
    Free,
    // Plus: The role of a user with a 'plus' status.
    #[iden = "Plus"]
    Plus,
    // Love: The role of a user with a 'love' status.
    #[iden = "Love"]
    Love,
    // Pro: The role of a user with a 'pro' status.
    #[iden = "Pro"]
    Pro,
    // Premium: The role of a user with a 'premium' status.
    #[iden = "Premium"]
    Premium,
    // Gold: The role of a user with a 'gold' status.
    #[iden = "Gold"]
    Gold,
    // Admin: The role of a user with admin privileges.
    #[iden = "Admin"]
    Admin,
}
