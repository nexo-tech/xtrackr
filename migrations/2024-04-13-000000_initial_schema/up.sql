-- Create visitors table
CREATE TABLE visitors (
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    consent_given BOOLEAN NOT NULL DEFAULT false,
    consent_updated_at TIMESTAMPTZ
);

-- Create sessions table
CREATE TABLE sessions (
    id UUID PRIMARY KEY,
    visitor_id UUID NOT NULL REFERENCES visitors(id),
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ended_at TIMESTAMPTZ,
    user_agent TEXT,
    ip_address TEXT
);

-- Create events table
CREATE TABLE events (
    id UUID PRIMARY KEY,
    session_id UUID NOT NULL REFERENCES sessions(id),
    event_type TEXT NOT NULL,
    path TEXT NOT NULL,
    user_agent TEXT,
    referrer TEXT,
    ip_address TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb
);

-- Create indexes
CREATE INDEX idx_visitors_consent ON visitors(consent_given);
CREATE INDEX idx_sessions_visitor_id ON sessions(visitor_id);
CREATE INDEX idx_events_session_id ON events(session_id);
CREATE INDEX idx_events_created_at ON events(created_at);

-- Create function to anonymize IP addresses
CREATE OR REPLACE FUNCTION anonymize_ip(ip TEXT) RETURNS TEXT AS $$
BEGIN
    IF ip IS NULL THEN
        RETURN NULL;
    END IF;
    
    -- For IPv4, set last octet to 0
    IF ip ~ '^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$' THEN
        RETURN regexp_replace(ip, '\.\d+$', '.0');
    END IF;
    
    -- For IPv6, return first 64 bits
    IF ip ~ ':' THEN
        RETURN regexp_replace(ip, ':[^:]+$', ':0');
    END IF;
    
    RETURN ip;
END;
$$ LANGUAGE plpgsql; 
