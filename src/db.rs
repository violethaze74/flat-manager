use actix::prelude::*;
use actix_web::*;
use diesel;
use diesel::prelude::*;
use diesel::result::{Error as DieselError};

use models;
use models::{DbExecutor};
use errors::ApiError;
use schema;

#[derive(Deserialize, Debug)]
pub struct CreateBuild {
}

impl Message for CreateBuild {
    type Result = Result<models::Build, ApiError>;
}

impl Handler<CreateBuild> for DbExecutor {
    type Result = Result<models::Build, ApiError>;

    fn handle(&mut self, _msg: CreateBuild, _: &mut Self::Context) -> Self::Result {
        use self::schema::builds::dsl::*;
        let conn = &self.0.get().unwrap();
        diesel::insert_into(builds)
            .default_values()
            .get_result::<models::Build>(conn)
            .map_err(|e| {
                From::from(e)
            })
    }
}


#[derive(Deserialize, Debug)]
pub struct CreateBuildRef {
    pub data : models::NewBuildRef,
}

impl Message for CreateBuildRef {
    type Result = Result<models::BuildRef, ApiError>;
}

impl Handler<CreateBuildRef> for DbExecutor {
    type Result = Result<models::BuildRef, ApiError>;

    fn handle(&mut self, msg: CreateBuildRef, _: &mut Self::Context) -> Self::Result {
        use self::schema::build_refs::dsl::*;
        let conn = &self.0.get().unwrap();
        diesel::insert_into(build_refs)
            .values(&msg.data)
            .get_result::<models::BuildRef>(conn)
            .map_err(|e| {
                From::from(e)
            })
    }
}

#[derive(Deserialize, Debug)]
pub struct LookupBuild {
    pub id: i32
}

impl Message for LookupBuild {
    type Result = Result<models::Build, ApiError>;
}

impl Handler<LookupBuild> for DbExecutor {
    type Result = Result<models::Build, ApiError>;

    fn handle(&mut self, msg: LookupBuild, _: &mut Self::Context) -> Self::Result {
        use schema::builds::dsl::*;
        let conn = &self.0.get().unwrap();
        builds
            .filter(id.eq(msg.id))
            .get_result::<models::Build>(conn)
            .map_err(|e| {
                From::from(e)
            })
    }
}

#[derive(Deserialize, Debug)]
pub struct LookupBuildRef {
    pub id: i32,
    pub ref_id: i32,
}

impl Message for LookupBuildRef {
    type Result = Result<models::BuildRef, ApiError>;
}

impl Handler<LookupBuildRef> for DbExecutor {
    type Result = Result<models::BuildRef, ApiError>;

    fn handle(&mut self, msg: LookupBuildRef, _: &mut Self::Context) -> Self::Result {
        use schema::build_refs::dsl::*;
        let conn = &self.0.get().unwrap();
        build_refs
            .filter(build_id.eq(msg.id))
            .filter(id.eq(msg.ref_id))
            .get_result::<models::BuildRef>(conn)
            .map_err(|e| From::from(e))
    }
}

#[derive(Deserialize, Debug)]
pub struct LookupBuildRefs {
    pub id: i32,
}

impl Message for LookupBuildRefs {
    type Result = Result<Vec<models::BuildRef>, ApiError>;
}

impl Handler<LookupBuildRefs> for DbExecutor {
    type Result = Result<Vec<models::BuildRef>, ApiError>;

    fn handle(&mut self, msg: LookupBuildRefs, _: &mut Self::Context) -> Self::Result {
        use schema::build_refs::dsl::*;
        let conn = &self.0.get().unwrap();
        build_refs
            .filter(build_id.eq(msg.id))
            .get_results::<models::BuildRef>(conn)
            .map_err(|e| From::from(e))
    }
}

#[derive(Deserialize, Debug)]
pub struct CommitBuild {
    pub id: i32,
}

impl Message for CommitBuild {
    type Result = Result<models::Build, ApiError>;
}

impl Handler<CommitBuild> for DbExecutor {
    type Result = Result<models::Build, ApiError>;

    fn handle(&mut self, msg: CommitBuild, _: &mut Self::Context) -> Self::Result {
        use schema::builds::dsl::*;
        let conn = &self.0.get().unwrap();
        conn.transaction::<models::Build, DieselError, _>(|| {
            let current_build = builds
                .filter(id.eq(msg.id))
                .get_result::<models::Build>(conn)?;
            if current_build.repo_state != models::RepoState::Uploading as i16 {
                return Err(DieselError::RollbackTransaction)
            }
            diesel::update(builds)
                .filter(id.eq(msg.id))
                .set((repo_state.eq(models::RepoState::Verifying as i16),))
                .get_result::<models::Build>(conn)
        })
            .map_err(|e| {
                match e {
                    DieselError::RollbackTransaction => ApiError::BadRequest("Build is already commited".to_string()),
                    _ => From::from(e)
                }
            })
    }
}
