---
---
--- Table:plan_factory
---

CREATE SEQUENCE IF NOT EXISTS plan_id_seq;
CREATE TABLE IF NOT EXISTS plan_factory (id bigint PRIMARY KEY DEFAULT next_id_v1('plan_id_seq'), type_meta JSONB, object_meta JSONB, category text, VERSION text, icon text, description text, status JSONB, created_at timestamptz DEFAULT now());

---
--- Table:plan_factory:create
---
CREATE
OR REPLACE FUNCTION insert_plan_factory_v1 (type_meta JSONB, object_meta JSONB, category text, VERSION text, icon text, description text, status JSONB) RETURNS SETOF plan_factory AS $$
BEGIN
   RETURN QUERY
   INSERT INTO
      plan_factory(type_meta, object_meta, category, version, icon, description, status)
   VALUES
      (
         type_meta, object_meta, category, version, icon, description, status
      )
      RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:plan_factory:select_or_insert
---
CREATE
OR REPLACE FUNCTION select_or_insert_plan_v1 (pname text, ptype_meta JSONB, pobject_meta JSONB, pcategory text, pversion text, picon text, pdescription text, pstatus JSONB) RETURNS SETOF plan_factory AS $$
DECLARE existing_plan plan_factory % rowtype;
BEGIN
   SELECT
      * INTO existing_plan
   FROM
      plan_factory
   WHERE
      object_meta ->> 'name' = pname
      AND version = pversion;
IF FOUND
THEN
   RETURN QUERY
   UPDATE
      plan_factory
   SET
      type_meta = ptype_meta, object_meta = pobject_meta, category = pcategory, icon = picon, description = pdescription, updated_at = now()
   WHERE
      object_meta ->> 'name' = pname
      AND version = pversion RETURNING *;
ELSE
   RETURN QUERY
   INSERT INTO
      plan_factory(type_meta, object_meta, category, version, icon, description, status)
   VALUES
      (
         ptype_meta, pobject_meta, pcategory, pversion, picon, pdescription, pstatus
      )
      ON CONFLICT DO NOTHING RETURNING *;
END
IF;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:plan_factory:show
---
CREATE
OR REPLACE FUNCTION get_plan_v1(pid bigint) RETURNS SETOF plan_factory AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      plan_factory
   WHERE
      id = pid;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:plan_factory:list_blank
---
CREATE
OR REPLACE FUNCTION get_plans_v1() RETURNS SETOF plan_factory AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      plan_factory;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:plan_factory:update_status
---
CREATE
OR REPLACE FUNCTION set_plan_status_v1 (pid bigint, plan_status JSONB) RETURNS SETOF plan_factory AS $$
BEGIN
   RETURN QUERY
   UPDATE
      plan_factory
   SET
      status = plan_status,
      updated_at = now()
   WHERE
      id = pid RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;
