CREATE TABLE IF NOT EXISTS states (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    name VARCHAR UNIQUE NOT NULL,
    description VARCHAR NOT NULL,
    progress INTEGER UNIQUE NOT NULL
);
