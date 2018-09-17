---
---
---
--- Table:origins, origin_members, team_members
---
CREATE SEQUENCE IF NOT EXISTS origin_id_seq;
CREATE TABLE IF NOT EXISTS origins (id bigint UNIQUE PRIMARY KEY DEFAULT next_id_v1('origin_id_seq'), name text UNIQUE, type_meta JSONB, object_meta JSONB, created_at timestamptz DEFAULT now(), updated_at timestamptz);
CREATE SEQUENCE IF NOT EXISTS origin_mem_id_seq;
CREATE TABLE IF NOT EXISTS origin_members (id bigint PRIMARY KEY DEFAULT next_id_v1('origin_mem_id_seq'), type_meta JSONB, object_meta JSONB, meta_data JSONB, created_at timestamptz DEFAULT now(), updated_at timestamptz);
CREATE SEQUENCE IF NOT EXISTS team_mem_id_seq;
CREATE TABLE IF NOT EXISTS team_members (id bigint PRIMARY KEY DEFAULT next_id_v1('team_mem_id_seq'), type_meta JSONB, object_meta JSONB, meta_data JSONB, created_at timestamptz DEFAULT now(), updated_at timestamptz);

---
--- Table:origins:list_blank
---
CREATE
OR REPLACE FUNCTION insert_origin_member_v1 (om_type_meta JSONB, om_obj_meta JSONB, om_meta_data JSONB) RETURNS SETOF origin_members AS $$
DECLARE existing_origin_member origin_members % rowtype;
inserted_origin_member origin_members;
BEGIN
   SELECT
      * INTO existing_origin_member
   FROM
      origin_members
   WHERE
      object_meta ->> 'account' = om_obj_meta ->> 'account' AND
      meta_data ->> 'origin' = om_meta_data ->> 'origin';
IF FOUND
THEN
   RETURN NEXT existing_origin_member;
ELSE
   INSERT INTO
         origin_members(type_meta, object_meta, meta_data)
      VALUES
      (
         om_type_meta, om_obj_meta, om_meta_data
      )
      RETURNING * into inserted_origin_member;
RETURN NEXT inserted_origin_member;
RETURN;
END
IF;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

CREATE
OR REPLACE FUNCTION internal_insert_origin_member_v1 (om_type_meta JSONB, om_obj_meta JSONB, om_meta_data JSONB) RETURNS void AS $$
BEGIN
   INSERT INTO
      origin_members ( type_meta, object_meta, meta_data)
   VALUES
      (
         om_type_meta,
         om_obj_meta,
         om_meta_data
      )
;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:origins:create stub origin named `rioos_system`
---
INSERT INTO
   origins (name, object_meta, type_meta)
VALUES
   (
      'rioos_system',
      '{"name":"rioos_system", "labels": {}, "account": "", "created_at": "", "deleted_at": "", "finalizers": [], "annotations": {}, "cluster_name": "", "initializers": {"result": {"code": 0, "reason": "", "status": "", "details": {"uid": "", "kind": "", "name": "", "group": "", "causes": [], "retry_after_seconds": 0}, "message": "", "type_meta": {"kind": "", "api_version": ""}}, "pending": []}, "owner_references": [{"uid": "", "kind": "", "name": "", "api_version": "", "block_owner_deletion": false}], "deletion_grace_period_seconds": 0}',
      '{"kind":"Origin","api_version":"v1"}'
   )
   ON CONFLICT (name) DO NOTHING;

---
--- Table:origins:create
---
CREATE
OR REPLACE FUNCTION insert_origin_v1 (origin_name text, origin_type_meta JSONB, origin_object_meta JSONB) RETURNS SETOF origins AS $$
DECLARE existing_origin origins % rowtype;
inserted_origin origins;
BEGIN
   SELECT
      * INTO existing_origin
   FROM
      origins
   WHERE
      name = origin_name LIMIT 1;
IF FOUND
THEN
   RETURN NEXT existing_origin;
ELSE
   INSERT INTO
      origins (name, type_meta, object_meta)
   VALUES
      (
         origin_name, origin_type_meta, origin_object_meta
      )
      ON CONFLICT (name) DO NOTHING RETURNING * into inserted_origin;
PERFORM internal_insert_origin_member_v1('{"kind":"OriginMember","api_version":"v1"}', origin_object_meta, json_build_object('origin', inserted_origin.name)::jsonb);
RETURN NEXT inserted_origin;
RETURN;
END
IF;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

CREATE
OR REPLACE FUNCTION insert_team_member_v1 (om_type_meta JSONB, om_obj_meta JSONB, om_meta_data JSONB) RETURNS SETOF team_members AS $$
DECLARE existing_member team_members % rowtype;
inserted_member team_members;
BEGIN
   SELECT
      * INTO existing_member
   FROM
      team_members
   WHERE
      object_meta ->> 'account' = om_obj_meta ->> 'account' AND
      meta_data ->> 'origin' = om_meta_data ->> 'origin' AND
      meta_data ->> 'team' = om_meta_data ->> 'team';
IF FOUND
THEN
   RETURN NEXT existing_member;
ELSE
   INSERT INTO
         team_members(type_meta, object_meta, meta_data)
      VALUES
      (
         om_type_meta, om_obj_meta, om_meta_data
      )
      RETURNING * into inserted_member;
RETURN NEXT inserted_member;
RETURN;
END
IF;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

CREATE
OR REPLACE FUNCTION direct_insert_team_member_v1 (om_type_meta JSONB, om_obj_meta JSONB, om_meta_data JSONB) RETURNS void AS $$
BEGIN
   INSERT INTO
      team_members ( type_meta, object_meta, meta_data)
   VALUES
      (
         om_type_meta, om_obj_meta, om_meta_data
      ) ;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:team_members:list_by_origins
---
CREATE
OR REPLACE FUNCTION get_team_members_by_origins_v1 (org_id text, account_id text) RETURNS SETOF team_members AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      team_members
   WHERE
      meta_data ->> 'origin' = org_id
      AND object_meta ->> 'account' = account_id;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

--- Table:origins:list_blank
---
CREATE
OR REPLACE FUNCTION get_origins_v1() RETURNS SETOF origins AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      origins;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:origins:show_by_name
---
CREATE
OR REPLACE FUNCTION get_origin_v1 (org_name text) RETURNS SETOF origins AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      origins
   WHERE
      name = org_name;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:origins:list_by_account
---

CREATE
OR REPLACE FUNCTION get_origin_by_account_v1 (account_id text) RETURNS SETOF origins AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      origins
   WHERE
      object_meta ->> 'account' = account_id;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;


---
--- Table:origins:list_by_account
---

CREATE
OR REPLACE FUNCTION get_origin_members_by_account_v1 (account_id text) RETURNS SETOF origin_members AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      origin_members
   WHERE
      object_meta ->> 'account' = account_id;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;
