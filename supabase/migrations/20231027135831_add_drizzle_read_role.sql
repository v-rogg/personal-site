REVOKE SELECT ON signatures FROM drizzle_read;
DROP ROLE IF EXISTS drizzle_read;
CREATE ROLE drizzle_read WITH PASSWORD 'readonly' LOGIN; -- Change password after migration
GRANT SELECT ON signatures TO drizzle_read;
