---
--- Table:services
---
CREATE SEQUENCE IF NOT EXISTS serv_id_seq;
CREATE TABLE IF NOT EXISTS services (id bigint PRIMARY KEY DEFAULT next_id_v1('serv_id_seq'), spec JSONB, metadata JSONB, status JSONB, object_meta JSONB, type_meta JSONB, updated_at timestamptz, created_at timestamptz DEFAULT now());

---
--- Table:services:create
---
CREATE 
OR REPLACE FUNCTION insert_services_v1 (spec JSONB, metadata JSONB, status JSONB, object_meta JSONB, type_meta JSONB) RETURNS SETOF services AS $$ 
BEGIN
   RETURN QUERY 
   INSERT INTO
      services(spec, metadata, status, object_meta, type_meta) 
   VALUES
      (
         spec,
         metadata,
         status,
         object_meta,
         type_meta 
      )
      RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:services:show
---
CREATE 
OR REPLACE FUNCTION get_services_v1 (sid bigint) RETURNS SETOF services AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      * 
   FROM
      services 
   WHERE
      id = sid;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:services:list_blank
---
CREATE 
OR REPLACE FUNCTION get_services_list_v1() RETURNS SETOF services AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      * 
   FROM
      services;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:services:show_by_assembly_factory
---
CREATE 
OR REPLACE FUNCTION get_services_by_assembly_factory_v1(pid text) RETURNS SETOF services AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      * 
   FROM
      services 
   WHERE
      object_meta @> json_build_object('owner_references', json_build_array(json_build_object('uid', pid)))::jsonb;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:services:update
---
CREATE 
OR REPLACE FUNCTION update_servive_by_v1 (sid bigint, spec_data JSONB, serv_metadata JSONB, status_data JSONB, object_meta_data JSONB) RETURNS SETOF services AS $$ 
BEGIN
   RETURN QUERY 
   UPDATE
      services 
   SET
      spec = spec_data,
      metadata = serv_metadata,
      status = status_data,
      object_meta = object_meta_data,
      updated_at = now() 
   WHERE
      id = sid RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

