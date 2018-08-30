---
---
--- Table:assemblys
---
CREATE SEQUENCE IF NOT EXISTS asm_id_seq;
CREATE TABLE IF NOT EXISTS assemblys (id bigint PRIMARY KEY DEFAULT next_id_v1('asm_id_seq'), type_meta JSONB, object_meta JSONB, selector text[], status JSONB, metadata JSONB, updated_at timestamptz, created_at timestamptz DEFAULT now());

---
--- Table:assemblys:create
---
CREATE
OR REPLACE FUNCTION insert_assembly_v1 (type_meta JSONB, object_meta JSONB, selector text[], status JSONB, metadata JSONB) RETURNS SETOF assemblys AS $$
BEGIN
   RETURN QUERY
   INSERT INTO
      assemblys(type_meta, object_meta, selector, status, metadata, updated_at)
   VALUES
      (
         type_meta, object_meta, selector, status, metadata, now()
      )
      RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:assemblys:show
---
CREATE
OR REPLACE FUNCTION get_assembly_v1 (aid bigint) RETURNS SETOF assemblys AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      assemblys
   WHERE
      id = aid;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:assemblys.show
---
CREATE
OR REPLACE FUNCTION get_assemblys_v1() RETURNS SETOF assemblys AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      assemblys;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:assemblys:show_for_assemblyfactory
---
CREATE
OR REPLACE FUNCTION get_assemblys_by_parentid_v1 (pid text) RETURNS SETOF assemblys AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      assemblys
   WHERE
      object_meta @> json_build_object('owner_references', json_build_array(json_build_object('uid', pid)))::jsonb;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;
CREATE
OR REPLACE FUNCTION update_assembly_v1 (aid bigint, asm_selector text[], asm_object_meta JSONB, asm_metadata JSONB) RETURNS SETOF assemblys AS $$
BEGIN
   RETURN QUERY
   UPDATE
      assemblys
   SET
      selector = asm_selector,
      object_meta = asm_object_meta,
      metadata = asm_metadata,
      updated_at = now()
   WHERE
      id = aid RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:assemblys:update_status with updated_at checking
---
CREATE
OR REPLACE FUNCTION set_assembly_status_v2 (aid bigint,updat timestamptz, asm_status JSONB) RETURNS SETOF assemblys AS $$
BEGIN
   RETURN QUERY
   UPDATE
      assemblys
   SET
      status = asm_status,
      updated_at = now()
   WHERE
      id = aid and updated_at = updat RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:assemblys:for_account
---
CREATE
OR REPLACE FUNCTION get_assemblys_by_account_v1 (account_id text) RETURNS SETOF assemblys AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      assemblys
   WHERE
      object_meta ->> 'account' = account_id;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:assembly_factory
---
CREATE SEQUENCE IF NOT EXISTS asm_fact_id_seq;
CREATE TABLE IF NOT EXISTS assembly_factory (id bigint PRIMARY KEY DEFAULT next_id_v1('asm_fact_id_seq'), object_meta JSONB, type_meta JSONB, replicas smallint, resources JSONB, metadata JSONB, status JSONB, secret JSONB, PLAN bigint REFERENCES plan_factory(id), spec JSONB, updated_at timestamptz, created_at timestamptz DEFAULT now());

---
--- Table:assembly_factory:create
---
CREATE
OR REPLACE FUNCTION insert_assembly_factory_v1 (object_meta JSONB, type_meta JSONB, replicas smallint, resources JSONB, metadata JSONB, status JSONB, secret JSONB, PLAN bigint, spec JSONB) RETURNS SETOF assembly_factory AS $$
BEGIN
   RETURN QUERY
   INSERT INTO
      assembly_factory(object_meta, type_meta, replicas, resources, metadata, status, secret, plan, spec)
   VALUES
      (
         object_meta,
         type_meta,
         replicas,
         resources,
         metadata,
         status,
         secret,
         plan,
         spec
      )
      RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:assembly_factory:show
---
CREATE
OR REPLACE FUNCTION get_assembly_factory_v1 (aid bigint) RETURNS SETOF assembly_factory AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      assembly_factory
   WHERE
      id = aid;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:assemblyfactorys:show_by_stacksfactory
--- Describe all the assemblyfactorys for the stacksfactory id.
---
CREATE
OR REPLACE FUNCTION get_assemblyfactorys_by_parentid_v1 (pid text) RETURNS SETOF assembly_factory AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      assembly_factory
   WHERE
      object_meta @> json_build_object('owner_references', json_build_array(json_build_object('uid', pid)))::jsonb;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;



---
--- Table:assembly_factory:list_blank
---
CREATE
OR REPLACE FUNCTION get_assemblys_factory_v1() RETURNS SETOF assembly_factory AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      assembly_factory;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:assembly_factory:update_status
---
CREATE
OR REPLACE FUNCTION set_assembly_factorys_status_v1 (aid bigint, asm_fac_status JSONB) RETURNS SETOF assembly_factory AS $$
BEGIN
   RETURN QUERY
   UPDATE
      assembly_factory
   SET
      status = asm_fac_status,
      updated_at = now()
   WHERE
      id = aid RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;


---
--- Table:assembly_factory:update
---
CREATE
OR REPLACE FUNCTION set_assembly_factorys_v1 (aid bigint,asm_object_meta JSONB, asm_type_meta JSONB, asm_replicas smallint, asm_resources JSONB, asm_metadata JSONB, asm_secret JSONB, asm_PLAN bigint, asm_spec JSONB) RETURNS SETOF assembly_factory AS $$
BEGIN
   RETURN QUERY
   UPDATE
      assembly_factory
   SET
      object_meta = asm_object_meta,
      type_meta = asm_type_meta,
      replicas = asm_replicas,
      resources =asm_resources,
      metadata= asm_metadata,
      secret=asm_secret,
      PLAN=asm_PLAN,
      spec= asm_spec,
      updated_at = now()
   WHERE
      id = aid RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;


---
--- Table:assembly_factory:show_for_account
---
CREATE
OR REPLACE FUNCTION get_assembly_factory_by_account_v1 (account_id text) RETURNS SETOF assembly_factory AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      assembly_factory
   WHERE
      object_meta ->> 'account' = account_id;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:stacksfactory
---
CREATE SEQUENCE IF NOT EXISTS stacks_factory_id_seq;
CREATE TABLE IF NOT EXISTS stacks_factory (id bigint PRIMARY KEY DEFAULT next_id_v1('stacks_factory_id_seq'), object_meta JSONB, type_meta JSONB, replicas smallint, resources JSONB, metadata JSONB, status JSONB, secret JSONB, PLAN bigint REFERENCES plan_factory(id), spec JSONB, updated_at timestamptz, created_at timestamptz DEFAULT now());

---
--- Table:stacks_factory:create
---
CREATE
OR REPLACE FUNCTION insert_stacks_factory_v1 (object_meta JSONB, type_meta JSONB, replicas smallint, resources JSONB, metadata JSONB, status JSONB, secret JSONB, PLAN bigint, spec JSONB) RETURNS SETOF stacks_factory AS $$
BEGIN
   RETURN QUERY
   INSERT INTO
      stacks_factory(object_meta, type_meta, replicas, resources, metadata, status, secret, plan, spec)
   VALUES
      (
         object_meta,
         type_meta,
         replicas,
         resources,
         metadata,
         status,
         secret,
         plan,
         spec
      )
      RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:stacks_factory:show
---
CREATE
OR REPLACE FUNCTION get_stacks_factory_v1 (aid bigint) RETURNS SETOF stacks_factory AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      stacks_factory
   WHERE
      id = aid;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:stacks_factory:list_blank
---
CREATE
OR REPLACE FUNCTION get_stacks_factorys_v1() RETURNS SETOF stacks_factory AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      stacks_factory;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:stacks_factory:update_status
---
CREATE
OR REPLACE FUNCTION set_stacks_factorys_status_v1 (aid bigint, asm_fac_status JSONB) RETURNS SETOF stacks_factory AS $$
BEGIN
   RETURN QUERY
   UPDATE
      stacks_factory
   SET
      status = asm_fac_status,
      updated_at = now()
   WHERE
      id = aid RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:stacks_factory:show_for_account
---
CREATE
OR REPLACE FUNCTION get_stacks_factory_by_account_v1 (account_id text) RETURNS SETOF stacks_factory AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      stacks_factory
   WHERE
      object_meta ->> 'account' = account_id;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;
