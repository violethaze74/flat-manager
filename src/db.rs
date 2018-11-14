use actix::prelude::*;
use actix_web::*;
use diesel;
use diesel::prelude::*;
use diesel::result::{Error as DieselError};

use models;
use models::{DbExecutor,RepoState,PublishedState, CommitJob, PublishJob, JobKind};
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
pub struct LookupJob {
    pub id: i32
}

impl Message for LookupJob {
    type Result = Result<models::Job, ApiError>;
}

impl Handler<LookupJob> for DbExecutor {
    type Result = Result<models::Job, ApiError>;

    fn handle(&mut self, msg: LookupJob, _: &mut Self::Context) -> Self::Result {
        use schema::jobs::dsl::*;
        let conn = &self.0.get().unwrap();
        jobs
            .filter(id.eq(msg.id))
            .get_result::<models::Job>(conn)
            .map_err(|e| {
                From::from(e)
            })
    }
}

#[derive(Deserialize, Debug)]
pub struct LookupCommitJob {
    pub build_id: i32
}

impl Message for LookupCommitJob {
    type Result = Result<models::Job, ApiError>;
}

impl Handler<LookupCommitJob> for DbExecutor {
    type Result = Result<models::Job, ApiError>;

    fn handle(&mut self, msg: LookupCommitJob, _: &mut Self::Context) -> Self::Result {
        use schema::jobs::dsl::*;
        use schema::builds::dsl::*;
        let conn = &self.0.get().unwrap();
        jobs
            .inner_join(builds.on(commit_job_id.eq(schema::jobs::dsl::id.nullable())))
            .select(schema::jobs::all_columns)
            .filter(schema::builds::dsl::id.eq(msg.build_id))
            .get_result::<models::Job>(conn)
            .map_err(|e| {
                From::from(e)
            })
    }
}

#[derive(Deserialize, Debug)]
pub struct LookupPublishJob {
    pub build_id: i32
}

impl Message for LookupPublishJob {
    type Result = Result<models::Job, ApiError>;
}

impl Handler<LookupPublishJob> for DbExecutor {
    type Result = Result<models::Job, ApiError>;

    fn handle(&mut self, msg: LookupPublishJob, _: &mut Self::Context) -> Self::Result {
        use schema::jobs::dsl::*;
        use schema::builds::dsl::*;
        let conn = &self.0.get().unwrap();
        jobs
            .inner_join(builds.on(publish_job_id.eq(schema::jobs::dsl::id.nullable())))
            .select(schema::jobs::all_columns)
            .filter(schema::builds::dsl::id.eq(msg.build_id))
            .get_result::<models::Job>(conn)
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
pub struct ListBuilds {
}

impl Message for ListBuilds {
    type Result = Result<Vec<models::Build>, ApiError>;
}

impl Handler<ListBuilds> for DbExecutor {
    type Result = Result<Vec<models::Build>, ApiError>;

    fn handle(&mut self, _msg: ListBuilds, _: &mut Self::Context) -> Self::Result {
        use schema::builds::dsl::*;
        let conn = &self.0.get().unwrap();
        let (val, _) = RepoState::Purged.to_db();
        builds
            .filter(repo_state.ne(val))
            .get_results::<models::Build>(conn)
            .map_err(|e| {
                From::from(e)
            })
    }
}


#[derive(Deserialize, Debug)]
pub struct StartCommitJob {
    pub id: i32,
    pub endoflife: Option<String>,
}

impl Message for StartCommitJob {
    type Result = Result<models::Job, ApiError>;
}

impl Handler<StartCommitJob> for DbExecutor {
    type Result = Result<models::Job, ApiError>;

    fn handle(&mut self, msg: StartCommitJob, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get().unwrap();
        conn.transaction::<models::Job, DieselError, _>(|| {
            let current_build = schema::builds::table
                .filter(schema::builds::id.eq(msg.id))
                .get_result::<models::Build>(conn)?;
            let current_repo_state = RepoState::from_db(current_build.repo_state, &current_build.repo_state_reason);
            if !current_repo_state.same_state_as(&RepoState::Uploading) {
                return Err(DieselError::RollbackTransaction)
            };
            let (val, reason) = RepoState::to_db(&RepoState::Verifying);
            let job =
            diesel::insert_into(schema::jobs::table)
                .values(models::NewJob {
                    kind: JobKind::Commit.to_db(),
                    contents: json!(CommitJob {
                        build: msg.id,
                        endoflife: msg.endoflife
                    }),
                })
                .get_result::<models::Job>(conn)?;
            diesel::update(schema::builds::table)
                .filter(schema::builds::id.eq(msg.id))
                .set((schema::builds::commit_job_id.eq(job.id),
                      schema::builds::repo_state.eq(val),
                      schema::builds::repo_state_reason.eq(reason)))
                .get_result::<models::Build>(conn)?;
            Ok(job)
        })
            .map_err(|e| {
                match e {
                    DieselError::RollbackTransaction => ApiError::BadRequest("Build is already commited".to_string()),
                    _ => From::from(e)
                }
            })
    }
}


#[derive(Deserialize, Debug)]
pub struct StartPublishJob {
    pub id: i32,
}

impl Message for StartPublishJob {
    type Result = Result<models::Job, ApiError>;
}

impl Handler<StartPublishJob> for DbExecutor {
    type Result = Result<models::Job, ApiError>;

    fn handle(&mut self, msg: StartPublishJob, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get().unwrap();
        conn.transaction::<models::Job, DieselError, _>(|| {
            let current_build = schema::builds::table
                .filter(schema::builds::id.eq(msg.id))
                .get_result::<models::Build>(conn)?;
            let current_published_state = PublishedState::from_db(current_build.published_state, &current_build.published_state_reason);
            if !current_published_state.same_state_as(&PublishedState::Unpublished) {
                error!("Unexpected publishing state {:?}", current_published_state);
                return Err(DieselError::RollbackTransaction) // Already published
            };
            let current_repo_state = RepoState::from_db(current_build.repo_state, &current_build.repo_state_reason);
            if !current_repo_state.same_state_as(&RepoState::Ready) {
                error!("Unexpected repo state {:?}", current_repo_state);
                return Err(DieselError::RollbackTransaction) // Not commited correctly
            };
            let (val, reason) = PublishedState::to_db(&PublishedState::Publishing);
            let job =
                diesel::insert_into(schema::jobs::table)
                .values(models::NewJob {
                    kind: JobKind::Publish.to_db(),
                    contents: json!(PublishJob {
                        build: msg.id,
                    }),
                })
                .get_result::<models::Job>(conn)?;
            diesel::update(schema::builds::table)
                .filter(schema::builds::id.eq(msg.id))
                .set((schema::builds::publish_job_id.eq(job.id),
                      schema::builds::published_state.eq(val),
                      schema::builds::published_state_reason.eq(reason)))
                .get_result::<models::Build>(conn)?;
            Ok(job)
        })
            .map_err(|e| {
                match e {
                    DieselError::RollbackTransaction => ApiError::BadRequest("Invalid build state for publish".to_string()),
                    _ => From::from(e)
                }
            })
    }
}

#[derive(Deserialize, Debug)]
pub struct InitPurge {
    pub id: i32,
}

impl Message for InitPurge {
    type Result = Result<(), ApiError>;
}

impl Handler<InitPurge> for DbExecutor {
    type Result = Result<(), ApiError>;

    fn handle(&mut self, msg: InitPurge, _: &mut Self::Context) -> Self::Result {
        use schema::builds::dsl::*;
        let conn = &self.0.get().unwrap();
        conn.transaction::<(), DieselError, _>(|| {
            let current_build = builds
                .filter(id.eq(msg.id))
                .get_result::<models::Build>(conn)?;
            let current_repo_state = RepoState::from_db(current_build.repo_state, &current_build.repo_state_reason);
            let current_published_state = PublishedState::from_db(current_build.published_state, &current_build.published_state_reason);
            if current_repo_state.same_state_as(&RepoState::Verifying) ||
                current_repo_state.same_state_as(&RepoState::Purging) ||
                current_published_state.same_state_as(&PublishedState::Publishing) {
                    /* Only allow pruning when we're not working on the build repo */
                return Err(DieselError::RollbackTransaction)
            };
            let (val, reason) = RepoState::to_db(&RepoState::Purging);
            diesel::update(builds)
                .filter(id.eq(msg.id))
                .set((repo_state.eq(val),
                      repo_state_reason.eq(reason)))
                .execute(conn)?;
            Ok(())
        })
            .map_err(|e| {
                match e {
                    DieselError::RollbackTransaction => ApiError::BadRequest("Can't prune build while in use".to_string()),
                    _ => From::from(e)
                }
            })
    }
}

#[derive(Deserialize, Debug)]
pub struct FinishPurge {
    pub id: i32,
    pub error: Option<String>,
}

impl Message for FinishPurge {
    type Result = Result<models::Build, ApiError>;
}

impl Handler<FinishPurge> for DbExecutor {
    type Result = Result<models::Build, ApiError>;

    fn handle(&mut self, msg: FinishPurge, _: &mut Self::Context) -> Self::Result {
        use schema::builds::dsl::*;
        let conn = &self.0.get().unwrap();
        conn.transaction::<models::Build, DieselError, _>(|| {
            let current_build = builds
                .filter(id.eq(msg.id))
                .get_result::<models::Build>(conn)?;
            let current_repo_state = RepoState::from_db(current_build.repo_state, &current_build.repo_state_reason);
            if !current_repo_state.same_state_as(&RepoState::Purging) {
                return Err(DieselError::RollbackTransaction)
            };
            let new_state = match msg.error {
                None => RepoState::Purged,
                Some(err_string) => RepoState::Failed(format!("Failed to Purge build: {}", err_string)),
            };
            let (val, reason) = RepoState::to_db(&new_state);
            let new_build =
                diesel::update(builds)
                .filter(id.eq(msg.id))
                .set((repo_state.eq(val),
                      repo_state_reason.eq(reason)))
                .get_result::<models::Build>(conn)?;
            Ok(new_build)
        })
            .map_err(|e| {
                match e {
                    DieselError::RollbackTransaction => ApiError::BadRequest("Unexpected repo state, was not purging".to_string()),
                    _ => From::from(e)
                }
            })
    }
}
