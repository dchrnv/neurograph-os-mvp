-- NeuroGraph OS Database Initialization Script
-- Creates schemas, extensions, and initial configuration

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";  -- For text search
CREATE EXTENSION IF NOT EXISTS "btree_gin"; -- For GIN indexes

-- Create schemas
CREATE SCHEMA IF NOT EXISTS tokens;
CREATE SCHEMA IF NOT EXISTS graph;
CREATE SCHEMA IF NOT EXISTS experience;

-- Set search path
SET search_path TO public, tokens, graph, experience;

-- Create custom types
DO $$ BEGIN
    CREATE TYPE connection_direction AS ENUM ('incoming', 'outgoing', 'bidirectional');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Grant permissions
GRANT ALL PRIVILEGES ON SCHEMA tokens TO neurograph_user;
GRANT ALL PRIVILEGES ON SCHEMA graph TO neurograph_user;
GRANT ALL PRIVILEGES ON SCHEMA experience TO neurograph_user;

-- Create utility functions
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Success message
DO $$
BEGIN
    RAISE NOTICE 'NeuroGraph OS database initialized successfully';
    RAISE NOTICE 'Schemas created: tokens, graph, experience';
    RAISE NOTICE 'Extensions enabled: uuid-ossp, pg_trgm, btree_gin';
END $$;