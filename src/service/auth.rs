use time::OffsetDateTime;

use crate::{
    config::db::postgres::PgPool,
    dto::auth::{LoginUserInput, RegisterUserInput, UpdateUserInput},
    error::{Error, Result},
    model::{
        auth::{CreateUserData, UpdateUserData},
        User,
    },
    util::encryption,
};

pub(crate) struct AuthService;

impl AuthService {
    pub(crate) async fn sign_in(input: LoginUserInput, pool: &PgPool) -> Result<User> {
        let user = User::find_by_email(&input.email, pool).await?;

        if encryption::verify_password(input.password, user.password.to_owned()).await? {
            Ok(user)
        } else {
            Err(Error::WrongPassword)
        }
    }

    pub(crate) async fn sign_up(input: RegisterUserInput, pool: &PgPool) -> Result<User> {
        if User::find_by_name(&input.name, pool).await.is_ok() {
            return Err(Error::DuplicateUserName);
        }

        if User::find_by_email(&input.email, pool).await.is_ok() {
            return Err(Error::DuplicateUserEmail);
        }

        let now = OffsetDateTime::now_utc().into();

        let data = CreateUserData {
            name: input.name,
            email: input.email,
            password: encryption::hash_password(input.password).await?,
            created_at: now,
            updated_at: now,
        };

        User::create(data, pool).await
    }

    pub(crate) async fn update(old: User, input: UpdateUserInput, pool: &PgPool) -> Result<User> {
        let new_password = match input.new_password {
            Some(new_password) if new_password != input.password => Some(new_password),
            Some(_) => None,
            None => None,
        };

        if !encryption::verify_password(input.password, old.password.clone()).await? {
            return Err(Error::WrongPassword);
        }

        let name = match input.name {
            Some(name) if name != old.name => {
                if User::find_by_name(&name, pool).await.is_ok() {
                    return Err(Error::DuplicateUserName);
                } else {
                    Some(name)
                }
            }
            Some(_) => None,
            None => None,
        };

        let email = match input.email {
            Some(email) if email != old.email => {
                if User::find_by_name(&email, pool).await.is_ok() {
                    return Err(Error::DuplicateUserEmail);
                } else {
                    Some(email)
                }
            }
            Some(_) => None,
            None => None,
        };

        if new_password.is_none() && name.is_none() && email.is_none() {
            return Ok(old);
        }

        let password = match new_password {
            Some(password) => Some(encryption::hash_password(password).await?),
            None => None,
        };

        let data = UpdateUserData {
            name,
            email,
            password,
            updated_at: Some(OffsetDateTime::now_utc().into()),
        };

        User::update(old.id, data, pool).await
    }
}
