CREATE TABLE blocks (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL,
    CONSTRAINT fk_user
      FOREIGN KEY(user_id) 
	    REFERENCES users(id),
    blocked_id VARCHAR(255) NOT NULL,
    CONSTRAINT fk_blocked
      FOREIGN KEY(blocked_id) 
	    REFERENCES users(id)
);