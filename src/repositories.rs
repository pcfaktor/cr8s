use diesel::prelude::*;

use crate::models::{Crate, NewCrate, NewRustacean, Rustacean};
use crate::models::{NewRole, NewUser, NewUserRole, Role, User, UserRole};
use crate::schema::{crates, roles, rustaceans, user_roles, users};

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
        connection: &mut PgConnection,
        new_user: NewUser,
        role_codes: Vec<String>,
    ) -> QueryResult<User> {
        let user = diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(connection)?;
        for code in role_codes {
            if code.is_empty() {
                continue;
            }

            let new_user_role = {
                if let Ok(role) = RoleRepository::find_by_code(connection, code.to_owned()) {
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                } else {
                    let new_role = NewRole {
                        name: code.to_owned(),
                        code: code.to_owned(),
                    };
                    let role = RoleRepository::create(connection, new_role)?;
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                }
            };

            diesel::insert_into(user_roles::table)
                .values(new_user_role)
                .get_result::<UserRole>(connection)?;
        }

        Ok(user)
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

    pub fn delete(connection: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(user_roles::table.filter(user_roles::user_id.eq(id))).execute(connection)?;
        diesel::delete(users::table.find(id)).execute(connection)
    }
}

pub struct RoleRepository;

impl RoleRepository {
    pub fn find_by_code(connection: &mut PgConnection, code: String) -> QueryResult<Role> {
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
