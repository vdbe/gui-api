CREATE TABLE IF NOT EXISTS refreshtokens (
       id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
       token UUID DEFAULT gen_random_uuid() UNIQUE NOT NULL,
       user_id UUID NOT NULL,
       expiry_date TIMESTAMP NOT NULL,
       CONSTRAINT fk_user_id FOREIGN KEY(user_id) REFERENCES users(id)
);
