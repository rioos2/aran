---
---
--- Table:horizontal_scalings
---
CREATE SEQUENCE IF NOT EXISTS hs_id_seq;
CREATE TABLE IF NOT EXISTS horizontal_scalings (id bigint PRIMARY KEY DEFAULT next_id_v1('hs_id_seq'), scale_type text, state text, metadata JSONB, spec JSONB, status JSONB, object_meta JSONB, type_meta JSONB, updated_at timestamptz, created_at timestamptz DEFAULT now());

---
--- Table:horizontal_scalings:create
---
CREATE 
OR REPLACE FUNCTION insert_hs_v1 (scale_type text, state text, metadata JSONB, spec JSONB, status JSONB, object_meta JSONB, type_meta JSONB) RETURNS SETOF horizontal_scalings AS $$ 
BEGIN
   RETURN QUERY 
   INSERT INTO
      horizontal_scalings(scale_type, state, metadata, spec, status, object_meta, type_meta) 
   VALUES
      (
         scale_type, state, metadata, spec, status, object_meta, type_meta
      )
      RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:horizontal_scalings:show
---
CREATE 
OR REPLACE FUNCTION get_horizontal_scaling_v1(hid bigint) RETURNS SETOF horizontal_scalings AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      * 
   FROM
      horizontal_scalings 
   WHERE
      id = hid;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:horizontal_scalings:list_blank
---
CREATE 
OR REPLACE FUNCTION get_hs_v1() RETURNS SETOF horizontal_scalings AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      * 
   FROM
      horizontal_scalings;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:horizontal_scalings:show_by_assembly_factory
---
CREATE 
OR REPLACE FUNCTION get_scale_by_asmfacid_v1(asm_fac_id text) RETURNS SETOF horizontal_scalings AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      * 
   FROM
      horizontal_scalings 
   where
      object_meta @ > json_build_object('owner_references', json_build_array(json_build_object('uid', asm_fac_id)))::jsonb;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:horizontal_scalings:update_status
---
CREATE 
OR REPLACE FUNCTION set_hs_status_v1 (hid bigint, hs_status JSONB) RETURNS SETOF horizontal_scalings AS $$ 
BEGIN
   RETURN QUERY 
   UPDATE
      horizontal_scalings 
   SET
      status = hs_status,
      updated_at = now() 
   WHERE
      id = hid RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:horizontal_scalings:update
---
CREATE 
OR REPLACE FUNCTION update_hs_v1 (hid bigint, hs_scale_type text, hs_state text, hs_metadata JSONB, hs_spec JSONB, hs_status JSONB, hs_object_meta JSONB) RETURNS SETOF horizontal_scalings AS $$ 
BEGIN
   RETURN QUERY 
   UPDATE
      horizontal_scalings 
   SET
      scale_type = hs_scale_type,
      state = hs_state,
      metadata = hs_metadata,
      spec = hs_spec,
      status = hs_status,
      object_meta = hs_object_meta,
      updated_at = now() 
   WHERE
      id = hid RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:vertical_scalings:create
---
CREATE SEQUENCE IF NOT EXISTS vs_id_seq;
CREATE TABLE IF NOT EXISTS vertical_scalings (id bigint PRIMARY KEY DEFAULT next_id_v1('vs_id_seq'), scale_type text, state text, update_policy JSONB, metadata JSONB, spec JSONB, status JSONB, object_meta JSONB, type_meta JSONB, updated_at timestamptz, created_at timestamptz DEFAULT now());
CREATE 
OR REPLACE FUNCTION insert_vs_v1 (scale_type text, state text, update_policy JSONB, metadata JSONB, spec JSONB, status JSONB, object_meta JSONB, type_meta JSONB) RETURNS SETOF vertical_scalings AS $$ 
BEGIN
   RETURN QUERY 
   INSERT INTO
      vertical_scalings(scale_type, state, update_policy, metadata, spec, status, object_meta, type_meta) 
   VALUES
      (
         scale_type,
         state,
         update_policy,
         metadata,
         spec,
         status,
         object_meta,
         type_meta
      )
      RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:vertical_scalings:list_blank
---
CREATE 
OR REPLACE FUNCTION get_vs_v1() RETURNS SETOF vertical_scalings AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      * 
   FROM
      vertical_scalings;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:vertical_scalings:update_status
---
CREATE 
OR REPLACE FUNCTION set_vs_status_v1 (vid bigint, hs_status JSONB) RETURNS SETOF vertical_scalings AS $$ 
BEGIN
   RETURN QUERY 
   UPDATE
      vertical_scalings 
   SET
      status = hs_status,
      updated_at = now() 
   WHERE
      id = vid RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:vertical_scalings:show
---
CREATE 
OR REPLACE FUNCTION get_vertical_scaling_v1(vid bigint) RETURNS SETOF vertical_scalings AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      * 
   FROM
      vertical_scalings 
   WHERE
      id = vid;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:vertical:update
---
CREATE 
OR REPLACE FUNCTION update_vs_v1 (vid bigint, vs_scale_type text, vs_state text, vs_update_policy JSONB, vs_metadata JSONB, vs_spec JSONB, vs_status JSONB, vs_object_meta JSONB) RETURNS SETOF vertical_scalings AS $$ 
BEGIN
   RETURN QUERY 
   UPDATE
      vertical_scalings 
   SET
      scale_type = vs_scale_type,
      state = vs_state,
      update_policy = vs_update_policy,
      metadata = vs_metadata,
      spec = vs_spec,
      status = vs_status,
      object_meta = vs_object_meta,
      updated_at = now() 
   WHERE
      id = vid RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;