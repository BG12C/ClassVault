/// A User is the "face" of a client which contains informations about the name, homeworks, etc.
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone, Eq, Hash)]
pub struct User {
    pub name: String,
}
