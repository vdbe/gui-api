CREATE TABLE IF NOT EXISTS tasks (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    state UUID UNIQUE NOT NULL,
    created_by UUID UNIQUE NOT NULL,
    taken_by UUID UNIQUE,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    taken_at TIMESTAMP,
    completed_at TIMESTAMP,
    title VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    CONSTRAINT fk_state FOREIGN KEY(state) REFERENCES states(id),
    CONSTRAINT fk_created_by FOREIGN KEY(created_by) REFERENCES users(id),
    CONSTRAINT fk_taken_by FOREIGN KEY(taken_by) REFERENCES users(id)
);
