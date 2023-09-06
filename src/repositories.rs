use diesel::dsl::{now, IntervalDsl};
use diesel::prelude::*;
use rocket_db_pools::deadpool_redis::redis::RedisError;

use crate::auth::SESSION_LIFE_TIME;
use crate::models::{Crate, NewCrate, NewRustacean, RoleCode, Rustacean};
use crate::models::{NewRole, NewUser, NewUserRole, Role, User, UserRole};
use crate::rocket_routes::CacheConnection;
use crate::schema::{crates, roles, rustaceans, user_roles, users};
use rocket_db_pools::{deadpool_redis::redis::AsyncCommands, Connection};

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub fn find(connection: &mut PgConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result(connection)
    }

    pub fn find_multiple(connection: &mut PgConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table.limit(limit).load(connection)
    }

    pub fn create(
        connection: &mut PgConnection,
        new_rustacean: NewRustacean,
    ) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .get_result(connection)
    }

    pub fn update(
        connection: &mut PgConnection,
        id: i32,
        rustacean: Rustacean,
    ) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::name.eq(rustacean.name),
                rustaceans::email.eq(rustacean.email),
            ))
            .get_result(connection)
    }

    pub fn delete(connection: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(connection)
    }
}

pub struct CrateRepository;

impl CrateRepository {
    pub fn find_since(connection: &mut PgConnection, hours_since: i32) -> QueryResult<Vec<Crate>> {
        crates::table
            .filter(crates::created_at.ge(now - hours_since.seconds()))
            .order(crates::id.desc())
            .load::<Crate>(connection)
    }

    pub fn find(connection: &mut PgConnection, id: i32) -> QueryResult<Crate> {
        crates::table.find(id).get_result(connection)
    }

    pub fn find_multiple(connection: &mut PgConnection, limit: i64) -> QueryResult<Vec<Crate>> {
        crates::table.limit(limit).load(connection)
    }

    pub fn create(connection: &mut PgConnection, new_crate: NewCrate) -> QueryResult<Crate> {
        diesel::insert_into(crates::table)
            .values(new_crate)
            .get_result(connection)
    }

    pub fn update(connection: &mut PgConnection, id: i32, a_crate: Crate) -> QueryResult<Crate> {
        diesel::update(crates::table.find(id))
            .set((
                crates::rustacean_id.eq(a_crate.rustacean_id),
                crates::code.eq(a_crate.code),
                crates::name.eq(a_crate.name),
                crates::version.eq(a_crate.version),
                crates::description.eq(a_crate.description),
            ))
            .get_result(connection)
    }

    pub fn delete(connection: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(crates::table.find(id)).execute(connection)
    }
}

pub struct UserRepository;

impl UserRepository {
    pub fn create(
        c: &mut PgConnection,
        new_user: NewUser,
        role_codes: Vec<RoleCode>,
    ) -> QueryResult<User> {
        let user = diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(c)?;

        for role_code in role_codes {
            let new_user_role = {
                if let Ok(role) = RoleRepository::find_by_code(c, &role_code) {
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                } else {
                    let name = role_code.to_string();
                    let new_role = NewRole {
                        name: name,
                        code: role_code,
                    };
                    let role = RoleRepository::create(c, new_role)?;
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                }
            };

            diesel::insert_into(user_roles::table)
                .values(new_user_role)
                .get_result::<UserRole>(c)?;
        }

        Ok(user)
    }

    pub fn find_by_username(connection: &mut PgConnection, username: &String) -> QueryResult<User> {
        users::table
            .filter(users::username.eq(username))
            .first(connection)
    }

    pub fn find_with_roles(
        connection: &mut PgConnection,
    ) -> QueryResult<Vec<(User, Vec<(UserRole, Role)>)>> {
        let users = users::table.load(connection)?;
        let user_roles = user_roles::table
            .inner_join(roles::table)
            .load::<(UserRole, Role)>(connection)?
            .grouped_by(&users);
        Ok(users.into_iter().zip(user_roles).collect())
    }

    pub fn find(connection: &mut PgConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result(connection)
    }

    pub fn delete(connection: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(user_roles::table.filter(user_roles::user_id.eq(id))).execute(connection)?;
        diesel::delete(users::table.find(id)).execute(connection)
    }
}

pub struct RoleRepository;

impl RoleRepository {
    pub fn find_by_code(connection: &mut PgConnection, code: &RoleCode) -> QueryResult<Role> {
        roles::table.filter(roles::code.eq(code)).first(connection)
    }

    pub fn find_by_ids(connection: &mut PgConnection, ids: Vec<i32>) -> QueryResult<Vec<Role>> {
        roles::table
            .filter(roles::id.eq_any(ids))
            .get_results(connection)
    }

    pub fn find_by_user(connection: &mut PgConnection, user: &User) -> QueryResult<Vec<Role>> {
        let user_roles = UserRole::belonging_to(&user).get_results(connection)?;
        let role_ids = user_roles.iter().map(|ur: &UserRole| ur.role_id).collect();
        Self::find_by_ids(connection, role_ids)
    }

    pub fn create(connection: &mut PgConnection, role: NewRole) -> QueryResult<Role> {
        diesel::insert_into(roles::table)
            .values(role)
            .get_result::<Role>(connection)
    }
}

pub struct SessionRepository;

impl SessionRepository {
    pub async fn cache_session_id(
        session_id: &String,
        user_id: i32,
        mut cache: Connection<CacheConnection>,
    ) -> Result<(), RedisError> {
        cache
            .set_ex::<_, _, ()>(
                format!("sessions/{}", session_id),
                user_id,
                SESSION_LIFE_TIME,
            )
            .await
    }
}
