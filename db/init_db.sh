#!/bin/bash

# Create a temporary SQL file
TEMP_SQL_FILE=$(mktemp)

# Write the SQL commands to the temporary file
cat <<EOF > $TEMP_SQL_FILE
-- Create the users table
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    body TEXT,
    status VARCHAR(20) NOT NULL
);

-- Create the rooms table
CREATE TABLE IF NOT EXISTS rooms (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    body TEXT,
    admin VARCHAR(50) NOT NULL
);

-- Insert data into users
INSERT INTO users (id, name, body, status) VALUES
(1, 'User1', 'Some desc', 'connected')
ON CONFLICT (id) DO NOTHING,
(2, 'User2', 'Some desc', 'connected')
ON CONFLICT (id) DO NOTHING,
(3, 'User3', 'Some desc', 'disconnected')
ON CONFLICT (id) DO NOTHING,
(4, 'User4', 'Some desc', 'connected')
ON CONFLICT (id) DO NOTHING;

-- Insert data into rooms
INSERT INTO rooms (id, name, body, admin) VALUES
(1, 'Room creation', 'My room', 'User hey')
ON CONFLICT (id) DO NOTHING;
EOF

# Export the PGPASSWORD environment variable for password authentication
export PGPASSWORD=$POSTGRES_PASSWORD

# Execute the SQL commands
psql -h $POSTGRES_HOSTNAME -U $POSTGRES_USER -d $POSTGRES_DB -f $TEMP_SQL_FILE

# Remove the temporary SQL file
rm $TEMP_SQL_FILE

# Unset the PGPASSWORD environment variable
unset PGPASSWORD
