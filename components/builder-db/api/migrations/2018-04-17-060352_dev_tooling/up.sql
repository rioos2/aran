---
---
--- Table:build_configs
---
CREATE SEQUENCE IF NOT EXISTS build_config_id_seq;
CREATE TABLE IF NOT EXISTS build_configs (id bigint PRIMARY KEY DEFAULT next_id_v1('build_config_id_seq'), meta_data JSONB, spec JSONB, status JSONB, object_meta JSONB, type_meta JSONB, updated_at timestamptz, created_at timestamptz DEFAULT now());

---
--- Table:build_configs:create
---
CREATE
OR REPLACE FUNCTION insert_build_config_v1 (meta_data JSONB, spec JSONB, status JSONB, object_meta JSONB, type_meta JSONB) RETURNS SETOF build_configs AS $$
BEGIN
   RETURN QUERY
   INSERT INTO
      build_configs(meta_data, spec, status, object_meta, type_meta)
   VALUES
      (
         meta_data, spec, status, object_meta, type_meta
      )
      RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:build_configs:list_blank
---
CREATE
OR REPLACE FUNCTION get_build_configs_v1() RETURNS SETOF build_configs AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      build_configs;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:build_configs:show
---
CREATE
OR REPLACE FUNCTION get_build_config_v1(bid bigint) RETURNS SETOF build_configs AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      build_configs
   where
      id = bid;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:build_configs:show_for_assembly_factory
---
CREATE
OR REPLACE FUNCTION get_build_config_by_assembly_factory_v1(aid text) RETURNS SETOF build_configs AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      build_configs
   where
      object_meta @> json_build_object('owner_references', json_build_array(json_build_object('uid', aid)))::jsonb;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:build_configs:update
---
CREATE
OR REPLACE FUNCTION update_build_config_by_v1 (sid bigint, build_spec JSONB, build_status JSONB, build_meta_data JSONB, build_object_meta JSONB) RETURNS SETOF build_configs AS $$
BEGIN
   RETURN QUERY
   UPDATE
      build_configs
   SET
      spec = build_spec,
      status = build_status,
      meta_data = build_meta_data,
      object_meta = build_object_meta,
      updated_at = now()
   WHERE
      id = sid RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:build_configs:update_status
---
CREATE
OR REPLACE FUNCTION set_build_configs_status_v1 (bid bigint, bc_status JSONB) RETURNS SETOF build_configs AS $$
BEGIN
   RETURN QUERY
   UPDATE
      build_configs
   SET
      status = bc_status,
      updated_at = now()
   WHERE
      id = bid RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:builds
---
CREATE SEQUENCE IF NOT EXISTS build_id_seq;
CREATE TABLE IF NOT EXISTS builds (id bigint PRIMARY KEY DEFAULT next_id_v1('build_id_seq'), status JSONB, spec JSONB, object_meta JSONB, type_meta JSONB, updated_at timestamptz, created_at timestamptz DEFAULT now());

---
--- Table:builds:create
---
CREATE
OR REPLACE FUNCTION insert_build_v1 (status JSONB, spec JSONB, object_meta JSONB, type_meta JSONB) RETURNS SETOF builds AS $$
BEGIN
   RETURN QUERY
   INSERT INTO
      builds(status, spec, object_meta, type_meta)
   VALUES
      (
         status,
         spec,
         object_meta,
         type_meta
      )
      RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:builds:list_blank
---
CREATE
OR REPLACE FUNCTION get_builds_v1() RETURNS SETOF builds AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      builds;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:builds:show
---
CREATE
OR REPLACE FUNCTION get_build_v1(bid bigint) RETURNS SETOF builds AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      builds
   where
      id = bid;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:builds:show_build_for_build_config
---
CREATE
OR REPLACE FUNCTION get_build_by_build_config_v1(bid text) RETURNS SETOF builds AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      builds
   where
      object_meta @> json_build_object('owner_references', json_build_array(json_build_object('uid', bid)))::jsonb;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:builds:update
---
CREATE
OR REPLACE FUNCTION update_build_by_v1 (sid bigint, build_spec JSONB, build_status JSONB, build_object_meta JSONB) RETURNS SETOF builds AS $$
BEGIN
   RETURN QUERY
   UPDATE
      builds
   SET
      spec = build_spec,
      status = build_status,
      object_meta = build_object_meta,
      updated_at = now()
   WHERE
      id = sid RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:builds:update_status
---
CREATE
OR REPLACE FUNCTION update_build_status_by_v1 (bid bigint, build_status JSONB) RETURNS SETOF builds AS $$
BEGIN
   RETURN QUERY
   UPDATE
      builds
   SET
      status = build_status,
      updated_at = now()
   WHERE
      id = bid RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:image_references
---
CREATE SEQUENCE IF NOT EXISTS imageref_id_seq;
CREATE TABLE IF NOT EXISTS image_references (id bigint PRIMARY KEY DEFAULT next_id_v1('imageref_id_seq'), spec JSONB, status JSONB, object_meta JSONB, type_meta JSONB, updated_at timestamptz, created_at timestamptz DEFAULT now());

---
--- Table:image_references:create
---
CREATE
OR REPLACE FUNCTION insert_image_ref_v1 (status JSONB, spec JSONB, object_meta JSONB, type_meta JSONB) RETURNS SETOF image_references AS $$
BEGIN
   RETURN QUERY
   INSERT INTO
      image_references(spec, status, object_meta, type_meta)
   VALUES
      (
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
--- Table:image_references:show
---
CREATE
OR REPLACE FUNCTION get_image_ref_v1(iid bigint) RETURNS SETOF image_references AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      image_references
   where
      id = iid;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:image_references:show_by_build_config
---
CREATE
OR REPLACE FUNCTION get_image_ref_by_build_config_v1(aid text) RETURNS SETOF image_references AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      image_references
   where
      object_meta @> json_build_object('owner_references', json_build_array(json_build_object('uid', aid)))::jsonb;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:image_references:list_blank
---
CREATE
OR REPLACE FUNCTION get_image_ref_by_v1() RETURNS SETOF image_references AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      image_references;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:image_references:update
---
CREATE
OR REPLACE FUNCTION update_image_ref_by_v1 (iid bigint, image_spec JSONB, image_status JSONB, image_object_meta JSONB) RETURNS SETOF image_references AS $$
BEGIN
   RETURN QUERY
   UPDATE
      image_references
   SET
      spec = image_spec,
      status = image_status,
      object_meta = image_object_meta,
      updated_at = now()
   WHERE
      id = iid RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:image_marks
---
CREATE SEQUENCE IF NOT EXISTS image_marks_id_seq;
CREATE TABLE IF NOT EXISTS image_marks (id bigint PRIMARY KEY DEFAULT next_id_v1('image_marks_id_seq'), tags JSONB, generation bigint, conditions JSONB, lookup_policy bool, image JSONB, object_meta JSONB, type_meta JSONB, updated_at timestamptz, created_at timestamptz DEFAULT now());

---
--- Table:image_marks:create
---
CREATE
OR REPLACE FUNCTION insert_image_marks_v1 (tags JSONB, generation bigint, conditions JSONB, lookup_policy bool, image JSONB, object_meta JSONB, type_meta JSONB) RETURNS SETOF image_marks AS $$
BEGIN
   RETURN QUERY
   INSERT INTO
      image_marks(tags, generation, conditions, lookup_policy, image, object_meta, type_meta)
   VALUES
      (
         tags,
         generation,
         conditions,
         lookup_policy,
         image,
         object_meta,
         type_meta
      )
      RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:image_marks:show
---
CREATE
OR REPLACE FUNCTION get_image_mark_v1(iid bigint) RETURNS SETOF image_marks AS $$ 
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      image_marks
   where
      id = iid;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:image_marks:list_blank
---
CREATE
OR REPLACE FUNCTION get_image_marks_v1() RETURNS SETOF image_marks AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      image_marks;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:image_marks:update
---
CREATE
OR REPLACE FUNCTION update_image_marks_by_v1 (iid bigint, image_tags JSONB, image_generation bigint, image_conditions JSONB, image_look_up_policy bool, image_data JSONB, image_object_meta JSONB) RETURNS SETOF image_marks AS $$
BEGIN
   RETURN QUERY
   UPDATE
      image_marks
   SET
      tags = image_tags,
      generation = image_generation,
      conditions = image_conditions,
      lookup_policy = image_look_up_policy,
      image = image_data,
      object_meta = image_object_meta,
      updated_at = now()
   WHERE
      id = iid RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:image_marks:show_by_build
---
CREATE
OR REPLACE FUNCTION get_image_marks_by_build_v1(aid text) RETURNS SETOF image_marks AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      image_marks
   where
      object_meta @> json_build_object('owner_references', json_build_array(json_build_object('uid', aid)))::jsonb;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;
