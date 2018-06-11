---
---
---
--- Table:origins, origin_members, teams, team_members
---
CREATE SEQUENCE IF NOT EXISTS origin_id_seq;
CREATE TABLE IF NOT EXISTS origins (id bigint UNIQUE PRIMARY KEY DEFAULT next_id_v1('origin_id_seq'), name text UNIQUE, type_meta JSONB, object_meta JSONB, created_at timestamptz DEFAULT now(), updated_at timestamptz);
CREATE SEQUENCE IF NOT EXISTS origin_mem_id_seq;
CREATE TABLE IF NOT EXISTS origin_members (id bigint PRIMARY KEY DEFAULT next_id_v1('origin_mem_id_seq'), type_meta JSONB, object_meta JSONB, meta_data JSONB, created_at timestamptz DEFAULT now(), updated_at timestamptz);
CREATE SEQUENCE IF NOT EXISTS team_id_seq;
CREATE TABLE IF NOT EXISTS teams (id bigint UNIQUE PRIMARY KEY DEFAULT next_id_v1('team_id_seq'), name text UNIQUE, type_meta JSONB, object_meta JSONB, meta_data JSONB, created_at timestamptz DEFAULT now(), updated_at timestamptz);
CREATE SEQUENCE IF NOT EXISTS team_mem_id_seq;
CREATE TABLE IF NOT EXISTS team_members (id bigint PRIMARY KEY DEFAULT next_id_v1('team_mem_id_seq'), type_meta JSONB, object_meta JSONB, meta_data JSONB, created_at timestamptz DEFAULT now(), updated_at timestamptz);

---
--- Table:origins:list_blank
---
CREATE 
OR REPLACE FUNCTION insert_origin_member_v1 (om_type_meta JSONB, om_obj_meta JSONB, om_meta_data JSONB) RETURNS void AS $$ 
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
OR REPLACE FUNCTION insert_origin_v1 (origin_name text, origin_type_meta JSONB, origin_object_meta JSONB, origin_mem_type_meta JSONB) RETURNS SETOF origins AS $$ 
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
PERFORM insert_origin_member_v1(origin_mem_type_meta, origin_object_meta, json_build_object('origin', inserted_origin.name)::jsonb);
RETURN NEXT inserted_origin;
RETURN;
END
IF;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;
CREATE 
OR REPLACE FUNCTION insert_team_member_v1 (om_type_meta JSONB, om_obj_meta JSONB, om_meta_data JSONB) RETURNS void AS $$ 
BEGIN
   INSERT INTO
      team_members ( type_meta, object_meta, meta_data) 
   VALUES
      (
         om_type_meta, om_obj_meta, om_meta_data
      )
;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:teams:create
---
CREATE 
OR REPLACE FUNCTION insert_team_v1 (team_name text, origin text, team_object_meta JSONB, team_type_meta JSONB, team_meta_data JSONB, team_mem_type_meta JSONB) RETURNS SETOF teams AS $$ 
DECLARE existing_team teams % rowtype;
inserted_team teams;
BEGIN
   SELECT
      * INTO existing_team 
   FROM
      teams 
   WHERE
      name = team_name LIMIT 1;
IF FOUND 
THEN
   RETURN NEXT existing_team;
ELSE
   INSERT INTO
      teams (name, type_meta, object_meta, meta_data) 
   VALUES
      (
         team_name, team_type_meta, team_object_meta, team_meta_data
      )
      ON CONFLICT (name) DO NOTHING RETURNING * into inserted_team;
PERFORM insert_origin_member_v1(team_mem_type_meta, team_object_meta, json_build_object('team', inserted_team.name, 'origin', origin)::jsonb);
RETURN NEXT inserted_team;
RETURN;
END
IF;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;


---
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
--- Table:origin_members:show_members_for_origin
---
CREATE 
OR REPLACE FUNCTION list_origin_members_v1 (om_origin_id bigint) RETURNS TABLE(account_name text) AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      origin_members.account_name 
   FROM
      origin_members 
   WHERE
      origin_id = om_origin_id 
   ORDER BY
      account_name ASC;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:origin_members:check_if_an_account_exists_in_originmembers
---
CREATE 
OR REPLACE FUNCTION check_account_in_origin_members_v1 (om_origin_name text, om_account_id bigint) RETURNS TABLE(is_member bool) AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      true 
   FROM
      origin_members 
   WHERE
      origin_name = om_origin_name 
      AND account_id = om_account_id;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:origin_members:list_originmembers_by_accountis
---
CREATE 
OR REPLACE FUNCTION list_origin_by_account_id_v1 (o_account_id bigint) RETURNS TABLE(origin_name text) AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      origin_members.origin_name 
   FROM
      origin_members 
   WHERE
      account_id = o_account_id 
   ORDER BY
      origin_name ASC;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:account_origins
---
CREATE TABLE IF NOT EXISTS account_origins (account_id bigint, account_email text, origin_id bigint, origin_name text, created_at timestamptz DEFAULT now(), updated_at timestamptz, UNIQUE(account_id, origin_id));

---
--- Table:account_origins:create
---
CREATE 
OR REPLACE FUNCTION insert_account_origin_v1 (o_account_id bigint, o_account_email text, o_origin_id bigint, o_origin_name text) RETURNS void AS $$ 
BEGIN
   INSERT INTO
      account_origins (account_id, account_email, origin_id, origin_name) 
   VALUES
      (
         o_account_id,
         o_account_email,
         o_origin_id,
         o_origin_name
      )
;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:account_origins:show
---
CREATE 
OR REPLACE FUNCTION get_account_origins_v1 (in_account_id bigint) RETURNS SETOF account_origins AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      * 
   FROM
      account_origins 
   WHERE
      account_id = in_account_id;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;
