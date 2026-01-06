ALTER TABLE tasks add COLUMN status_updated_at TIMESTAMPTZ DEFAULT now();
