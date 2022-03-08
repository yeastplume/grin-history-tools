use crate::schema::*;
use chrono::*;
use bigdecimal::BigDecimal;

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "headers"]
pub struct Header {
    pub chain_type: i16,
    pub height: BigDecimal,
    pub version: i16,
    pub fork: i16,
    pub hash: Vec<u8>,
    pub kernel_mmr_size: BigDecimal,
    pub kernel_root: Vec<u8>,
    pub output_mmr_size: BigDecimal,
    pub output_root: Vec<u8>,
    pub prev_root: Vec<u8>,
    pub previous: Vec<u8>,
    pub range_proof_root: Vec<u8>,
    pub total_kernel_offset: Vec<u8>,
    pub timestamp_utc: NaiveDateTime,
    pub edge_bits: i16,
    pub total_difficulty: BigDecimal,
    pub nonce: BigDecimal,
    pub secondary_scaling: i64,
    pub cuckoo_solution: Vec<Vec<u8>>
}

/*#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub user_uuid: Uuid,
    pub hash: Vec<u8>,
    pub salt: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub name: String,
    pub role: String,
}

#[derive(Debug, Deserialize, juniper::GraphQLInputObject)]
pub struct UserData {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, juniper::GraphQLObject)]
pub struct SlimUser {
    pub user_uuid: Uuid,
    pub email: String,
    pub role: String,
}

#[derive(Shrinkwrap, Clone, Default)]
pub struct LoggedUser(pub Option<SlimUser>);

impl From<SlimUser> for LoggedUser {
    fn from(slim_user: SlimUser) -> Self {
        LoggedUser(Some(slim_user))
    }
}

impl From<UserData> for InsertableUser {
    fn from(user_data: UserData) -> Self {
        let UserData {
            name,
            email,
            password,
            ..
        } = user_data;

        let salt = make_salt();
        let hash = make_hash(&password, &salt).to_vec();
        Self {
            user_uuid: Uuid::new_v4(),
            email,
            hash,
            created_at: chrono::Local::now().naive_local(),
            salt,
            name,
            role: "user".to_owned(),
        }
    }
}
impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        let User {
            user_uuid,
            email,
            role,
            ..
        } = user;

        Self {
            user_uuid,
            email,
            role,
        }
    }
}
*/