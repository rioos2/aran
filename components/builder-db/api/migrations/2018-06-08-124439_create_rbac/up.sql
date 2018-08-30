CREATE SEQUENCE IF NOT EXISTS policies_id_seq;
CREATE TABLE IF NOT EXISTS policies (id bigint PRIMARY KEY DEFAULT next_id_v1('policies_id_seq'), type_meta JSONB, object_meta JSONB, metadata JSONB, description text, updated_at timestamptz, created_at timestamptz DEFAULT now());


CREATE SEQUENCE IF NOT EXISTS poli_mem_id_seq;
CREATE TABLE IF NOT EXISTS policy_members(id bigint PRIMARY KEY DEFAULT next_id_v1('poli_mem_id_seq'),type_meta JSONB, object_meta JSONB, metadata JSONB, is_allow text, policy_name text, created_at timestamptz DEFAULT now());

---
--- Table:policy_members:internal insert
---
CREATE OR REPLACE FUNCTION internal_insert_policy_member_v1(team_id bigint, account text, org_id text, allowed bool, acc_policy_name text) RETURNS SETOF policy_members AS $$
DECLARE inserted_policy_member policy_members;
BEGIN
   INSERT INTO
         policy_members(type_meta, object_meta, metadata, is_allow, policy_name)
      VALUES
      (
         '{"kind":"PolicyMemeber","api_version":"v1"}', json_build_object('account',account)::jsonb, json_build_object('team', team_id::text, 'origin', org_id::text)::jsonb, allowed, acc_policy_name
      )
      ON CONFLICT DO NOTHING RETURNING * into inserted_policy_member;
RETURN NEXT inserted_policy_member;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;


---
--- Table:policy_member:create
---
CREATE
OR REPLACE FUNCTION insert_policy_member_v1 (acc_policy_name text, allowed text, object_meta JSONB, type_meta JSONB, metadata JSONB) RETURNS SETOF policy_members AS $$
DECLARE inserted_policy_member policy_members;
BEGIN
   INSERT INTO
      policy_members(type_meta, object_meta, metadata, is_allow, policy_name)
   VALUES
      (
         type_meta, 
         object_meta, 
         metadata,
         allowed,
         acc_policy_name
      )
      ON CONFLICT DO NOTHING RETURNING * into inserted_policy_member;
      RETURN NEXT inserted_policy_member;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:policy_members:update_policy_member
---
CREATE
OR REPLACE FUNCTION update_policy_member_v1 (aid bigint, allowed text) RETURNS SETOF policy_members AS $$
BEGIN
   RETURN QUERY
   UPDATE
      policy_members
   SET      
      is_allow = allowed
   WHERE
      id = aid RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:policy_members:list_by_account
---
CREATE
OR REPLACE FUNCTION get_policy_members_by_account_v1 (account_id text) RETURNS SETOF policy_members AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      policy_members
   WHERE
      object_meta ->> 'account' = account_id;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:policy_members:list_by_account
---
CREATE
OR REPLACE FUNCTION get_policy_members_by_team_v1 (team_id text) RETURNS SETOF policy_members AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      policy_members
   WHERE
      metadata ->> 'team' = team_id;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;


---
--- Table:policies:list_blank
---
CREATE
OR REPLACE FUNCTION get_policies_v1 () RETURNS SETOF policies AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      policies;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:policies:list_by_level
---
CREATE
OR REPLACE FUNCTION get_policies_by_level_v1 (level text) RETURNS SETOF policies AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      policies
   WHERE
      metadata ->> 'level' = level;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:teams
---
CREATE SEQUENCE IF NOT EXISTS team_id_seq;
CREATE TABLE IF NOT EXISTS TEAMS (id bigint PRIMARY KEY DEFAULT next_id_v1('team_id_seq'), full_name text, type_meta JSONB, object_meta JSONB, metadata JSONB, description text, updated_at timestamptz, created_at timestamptz DEFAULT now(), UNIQUE (full_name));

---
--- Table:teams:create
---
CREATE
OR REPLACE FUNCTION insert_team_v1 (full_name text, description text,account text, origin text, object_meta JSONB, type_meta JSONB, metadata JSONB) RETURNS SETOF TEAMS AS $$
DECLARE inserted_teams teams;
BEGIN
   INSERT INTO
      teams(full_name, description, type_meta, object_meta, metadata)
   VALUES
      (
         full_name,
         description,
         type_meta, object_meta, metadata
      )
      ON CONFLICT DO NOTHING RETURNING * INTO inserted_teams;
      PERFORM direct_insert_team_member_v1('{"kind":"TeamMember","api_version":"v1"}', json_build_object('account',account)::jsonb,json_build_object('team', inserted_teams.id::text, 'origin', origin)::jsonb);
      RETURN NEXT inserted_teams;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;


---
--- Table:teams:list_blank
---
CREATE
OR REPLACE FUNCTION get_teams_v1 () RETURNS SETOF TEAMS AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      teams;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:teams:show
---
CREATE
OR REPLACE FUNCTION get_team_v1 (rid bigint) RETURNS SETOF TEAMS AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      teams
   WHERE
      id = rid;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:teams:show_by_name
---
CREATE
OR REPLACE FUNCTION get_team_by_name_v1 (rname text) RETURNS SETOF TEAMS AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      teams
   WHERE
      object_meta ->> 'name' = rname;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:teams:show_by_full_name
---
CREATE
OR REPLACE FUNCTION get_team_by_full_name_v1 (rname text) RETURNS SETOF TEAMS AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      teams
   WHERE
      full_name = rname;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:teams:list_by_origins
---
CREATE
OR REPLACE FUNCTION get_teams_by_origins_v1 (org_id text) RETURNS SETOF TEAMS AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      teams
   WHERE
      metadata ->> 'origin' = org_id ;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:permissions
---
CREATE SEQUENCE IF NOT EXISTS perm_id_seq;
CREATE TABLE IF NOT EXISTS permissions (id bigint PRIMARY KEY DEFAULT next_id_v1('perm_id_seq'), policy_id bigint REFERENCES policies(id), name text, description text, updated_at timestamptz, created_at timestamptz DEFAULT now());

---
--- Table:permissions:create
---
CREATE
OR REPLACE FUNCTION insert_permission_v1 (per_policy_id bigint, per_name text, per_description text) RETURNS SETOF permissions AS $$
BEGIN
   IF EXISTS
   (
      SELECT
         true
      FROM
         teams
      WHERE
         id = per_policy_id
   )
THEN
   RETURN QUERY
   INSERT INTO
      permissions (policy_id, name, description)
   VALUES
      (
         per_policy_id, per_name, per_description
      )
      ON CONFLICT DO NOTHING RETURNING *;
RETURN;
END
IF;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:permissions:list_blank
---
CREATE
OR REPLACE FUNCTION get_permissions_v1 () RETURNS SETOF permissions AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      permissions;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:permissions:show
---
CREATE
OR REPLACE FUNCTION get_permission_v1 (pid bigint) RETURNS SETOF permissions AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      permissions
   WHERE
      id = pid;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:permissions:show_permission_for_a_team
---
CREATE
OR REPLACE FUNCTION get_permission_by_policy_v1 (perm_id bigint, rid bigint) RETURNS SETOF permissions AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      permissions
   WHERE
      policy_id = rid
      AND id = perm_id
   ORDER BY
      name ASC;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:permissions:show_permissions_for_a_team
---
CREATE
OR REPLACE FUNCTION get_permissions_by_policy_v1 (rid bigint) RETURNS SETOF permissions AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      permissions
   WHERE
      policy_id = rid
   ORDER BY
      name ASC;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:permissions:show_permissions_for_a_team
---
CREATE
OR REPLACE FUNCTION get_permissions_by_policy_name_v1 (rname text) RETURNS SETOF permissions AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      permissions
   WHERE
      policy_id IN
      (
         SELECT
            id
         FROM
            policies
         WHERE
            object_meta ->> 'name' = rname
      )
   ORDER BY
      name ASC;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

-- --- future purpose
-- --- Table:permissions:show_permissions_for_an_user (account - email)
-- ---
-- CREATE
-- OR REPLACE FUNCTION get_permission_by_email_v1 (r_name text) RETURNS SETOF permissions AS $$
-- DECLARE existing_account accounts % rowtype;
-- BEGIN
--    SELECT
--       * INTO existing_account
--    FROM
--       accounts
--    WHERE
--       email = r_name LIMIT 1;
-- IF FOUND
-- THEN
--    RETURN QUERY
--    SELECT
--       *
--    FROM
--       permissions
--    WHERE
--       team_id IN
--       (
--          SELECT
--             id
--          FROM
--             teams
--          WHERE
--             name = ANY((
--             SELECT
--                teams
--             FROM
--                accounts
--             WHERE
--                email = r_name)::text[])
--       )
-- ;
-- RETURN;
-- ELSE
--    RETURN QUERY
--    SELECT
--       *
--    FROM
--       permissions
--    WHERE
--       team_id IN
--       (
--          SELECT
--             id
--          FROM
--             teams
--          WHERE
--             name = ANY((
--             SELECT
--                teams
--             FROM
--                service_accounts
--             WHERE
--                object_meta ->> 'name' = r_name)::text[])
--       )
-- ;
-- RETURN;
-- END
-- IF;
-- RETURN;
-- END
-- $$ LANGUAGE PLPGSQL STABLE;


---
--- Table:invitations
---
CREATE SEQUENCE IF NOT EXISTS invite_id_seq;
CREATE TABLE IF NOT EXISTS INVITATIONS (id bigint PRIMARY KEY DEFAULT next_id_v1('invite_id_seq'), invite_from text, invite_to text, type_meta JSONB, object_meta JSONB, origin_id text, status text, team_id text, updated_at timestamptz, created_at timestamptz DEFAULT now(), UNIQUE (id));

---
--- Table:invitations:create
---
CREATE
OR REPLACE FUNCTION insert_invitations_v1 (invite_from text, invite_to text, origin_id text, team_id text, object_meta JSONB, type_meta JSONB, status text) RETURNS SETOF INVITATIONS AS $$
BEGIN
   RETURN QUERY
   INSERT INTO
      invitations(invite_from, invite_to, origin_id, team_id, object_meta, type_meta, status)
   VALUES
      (
         invite_from,
         invite_to,
         origin_id,
         team_id,
         object_meta,
         type_meta,
         status
      )
      RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:teams:list_by_origins
---
CREATE
OR REPLACE FUNCTION get_invitations_by_teams_v1 (mteam_id text) RETURNS SETOF INVITATIONS AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      invitations
   WHERE
      team_id = mteam_id ;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:networks:show
---
CREATE
OR REPLACE FUNCTION get_invitations_v1(nid bigint) RETURNS SETOF invitations AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      invitations
   where
      id = nid;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

CREATE
OR REPLACE FUNCTION update_status_by_team_v1 (tid bigint, accept text) RETURNS SETOF INVITATIONS AS $$
BEGIN
   RETURN QUERY
   UPDATE
      invitations
   SET
      status = accept,
      updated_at = now()
   WHERE
      id = tid RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;
