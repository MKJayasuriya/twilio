-- 
CREATE TABLE twilio_webhook_logs (
    id SERIAL PRIMARY KEY,
    sms_sid VARCHAR(64) UNIQUE NOT NULL,
    from_phone VARCHAR(64) NOT NULL,
    to_phone VARCHAR(64) NOT NULL, 
    account_sid VARCHAR(64) NOT NULL,
    latest_status VARCHAR(64),
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE twilio_message_status (
    id SERIAL PRIMARY KEY,
    sms_sid VARCHAR(64) NOT NULL REFERENCES twilio_webhook_logs(sms_sid) ON DELETE CASCADE,
    message_status VARCHAR(64) NOT NULL,
    status_updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE twilio_incoming_messages (
    id SERIAL PRIMARY KEY,
    sms_message_sid VARCHAR(255) NOT NULL,
    num_media INT NOT NULL,
    profile_name VARCHAR(255),
    message_type VARCHAR(50),
    sms_sid VARCHAR(255) NOT NULL,
    wa_id VARCHAR(50),
    sms_status VARCHAR(50),
    body TEXT,
    button_text VARCHAR(255),
    to_phone VARCHAR(50),
    button_payload VARCHAR(255),
    num_segments INT,
    referral_num_media INT,
    message_sid VARCHAR(255),
    account_sid VARCHAR(255),
    from_phone VARCHAR(50),
    api_version VARCHAR(50),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
