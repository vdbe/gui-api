use std::time::SystemTime;

use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    config::db::postgres::PgPool,
    dto::auth::{LoginUserInput, RefreshTokenInput, RegisterUserInput, UpdateUserInput},
    error::{Error, Result},
    model::{
        auth::{CreateRefreshTokenData, CreateUserData, RefreshToken, UpdateUserData},
        User,
    },
    util::encryption,
};

pub(crate) struct AuthService;

impl AuthService {
    const REFRESH_TOKEN_TIMEOUT: u64 = 30 * 24 * 60 * 60;

    pub(crate) async fn create_refresh_token(user_id: Uuid, pool: &PgPool) -> Result<RefreshToken> {
        let data = CreateRefreshTokenData {
            user_id,
            expiry_date: SystemTime::now()
                + std::time::Duration::from_secs(Self::REFRESH_TOKEN_TIMEOUT),
        };

        RefreshToken::create(data, pool).await
    }

    pub(crate) async fn refresh_access_token(
        input: RefreshTokenInput,
        pool: &PgPool,
    ) -> Result<User> {
        let refresh_token = RefreshToken::find_by_token(input.token, pool).await?;

        if refresh_token.expiry_date < SystemTime::now() {
            return Err(Error::RefreshTokenExpired);
        }

        User::find_by_id(refresh_token.user_id, pool).await
    }

    pub(crate) async fn sign_in(input: LoginUserInput, pool: &PgPool) -> Result<User> {
        let user = User::find_by_email(&input.email, pool).await?;

        if encryption::verify_password(input.password, user.password.to_owned()).await? {
            Ok(user)
        } else {
            Err(Error::WrongPassword)
        }
    }

    pub(crate) async fn sign_out(input: RefreshTokenInput, pool: &PgPool) -> Result<()> {
        // NOTE: Maybe an error if the refreshtoken did not exist
        RefreshToken::drop_by_token(input.token, pool).await
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
