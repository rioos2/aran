---
--- Table:endpoints
---
CREATE SEQUENCE IF NOT EXISTS end_id_seq;
CREATE TABLE IF NOT EXISTS endpoints (id bigint PRIMARY KEY DEFAULT next_id_v1('end_id_seq'), subsets JSONB, object_meta JSONB, type_meta JSONB, updated_at timestamptz, created_at timestamptz DEFAULT now());

---
--- Table:endpoints:create
---
CREATE 
OR REPLACE FUNCTION insert_endpoints_v1 (subsets JSONB, object_meta JSONB, type_meta JSONB) RETURNS SETOF endpoints AS $$ 
BEGIN
   RETURN QUERY 
   INSERT INTO
      endpoints(subsets, object_meta, type_meta) 
   VALUES
      (
         subsets,
         object_meta,
         type_meta 
      )
      RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:endpoints:list_blank
---
CREATE 
OR REPLACE FUNCTION get_endpoints_v1() RETURNS SETOF endpoints AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      * 
   FROM
      endpoints;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:endpoints:show
---
CREATE 
OR REPLACE FUNCTION get_endpoint_v1 (eid bigint) RETURNS SETOF endpoints AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      * 
   FROM
      endpoints 
   WHERE
      id = eid;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:endpoints:show_by_account
---
CREATE 
OR REPLACE FUNCTION get_endpoints_by_account_v1(account_id text) RETURNS SETOF endpoints AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      * 
   FROM
      endpoints 
   WHERE
      object_meta ->> 'account' = account_id;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:endpoints:show_by_assembly
---
CREATE 
OR REPLACE FUNCTION get_endpoints_by_assebmly_v1(pid text) RETURNS SETOF endpoints AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      * 
   FROM
      endpoints 
   WHERE
      object_meta @> json_build_object('owner_references', json_build_array(json_build_object('uid', pid)))::jsonb;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;
