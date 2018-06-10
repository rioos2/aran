---
--- Table:service_accounts
---
CREATE SEQUENCE IF NOT EXISTS service_id_seq;
CREATE TABLE IF NOT EXISTS service_accounts(id bigint PRIMARY KEY DEFAULT next_id_v1('service_id_seq'), secrets JSONB, object_meta JSONB, type_meta JSONB, metadata JSONB, ROLES text[], updated_at timestamptz, created_at timestamptz DEFAULT now());

---
--- Table:service_accounts:create
---
CREATE 
OR REPLACE FUNCTION insert_service_account_v1 (secrets JSONB, object_meta JSONB, type_meta JSONB, metadata JSONB, ROLES text[]) RETURNS SETOF service_accounts AS $$ 
BEGIN
   RETURN QUERY 
   INSERT INTO
      service_accounts(secrets, object_meta, type_meta, metadata, roles) 
   VALUES
      (
         secrets,
         object_meta,
         type_meta,
         metadata,
         roles
      )
      RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:service_accounts:update
---
CREATE 
OR REPLACE FUNCTION update_service_account_v1 (aid bigint, sa_secrets JSONB, asm_object_meta JSONB) RETURNS SETOF service_accounts AS $$ 
BEGIN
   RETURN QUERY 
   UPDATE
      service_accounts 
   SET
      secrets = sa_secrets,
      object_meta = asm_object_meta,
      updated_at = now() 
   WHERE
      id = aid RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:service_accounts:list_blank
---
CREATE 
OR REPLACE FUNCTION get_service_account_v1() RETURNS SETOF service_accounts AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      * 
   FROM
      service_accounts;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:service_accounts:show
---
CREATE 
OR REPLACE FUNCTION get_service_account_by_id_v1 (sid bigint) RETURNS SETOF service_accounts AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      * 
   FROM
      service_accounts 
   WHERE
      id = sid;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:service_accounts:show_by_name
---
CREATE 
OR REPLACE FUNCTION get_serviceaccount_by_name_v1(ser_name text) RETURNS SETOF service_accounts AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      * 
   FROM
      service_accounts 
   WHERE
      object_meta ->> 'name' = ser_name;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;