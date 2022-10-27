#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "voters"]
pub struct Voter {
    pub first_name: String,
    pub last_name: String,
    pub party: String,
    pub salary: i32,
    pub age: i32,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Voters {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub party: String,
    pub salary: i32,
    pub age: i32,
}

impl Voters {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let voters = voters::table.load::<Voters>(&conn)?;
        Ok(voters)
    }
    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let voter = voters::table.filter(voters::id.eq(id)).first(&conn)?;
        Ok(voter)
    }
    pub fn create(voter: Voter) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let voter = Voter::from(voter);
        let voter = diesel::insert_into(voters::table)
            .values(voter)
            .get_result(&conn)?;
        Ok(voter)
    }
    pub fn update(id: i32, voter: Voter) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let voter = diesel::update(voters::table)
            .filter(voters::id.eq(id))
            .set(voter)
            .get_result(&conn)?;
        Ok(voter)
    }
    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(voters::table.filter(voters::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}
impl Voter {
    fn from(voter: Voter) -> Voter {
        Voter {
            first_name: voter.first_name,
            last_name: voter.last_name,
            party: voter.party,
            salary: voter.salary,
            age: voter.age,
        }
    }
}