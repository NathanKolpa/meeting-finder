use common::model::Organization;

pub fn map_org_to_logo_url(org: &Organization) -> &'static str {
    match org {
        Organization::AnonymousAlcoholics => "/logos/aa.png",
        Organization::DebtorsAnonymous => "/logos/da.png",
        Organization::CrystalMethAnonymous => "/logos/cma_resize.webp",
        Organization::CodependentsAnonymous => "/logos/coda.png",
        Organization::NarcoticsAnonymous => "/logos/na.png",
    }
}