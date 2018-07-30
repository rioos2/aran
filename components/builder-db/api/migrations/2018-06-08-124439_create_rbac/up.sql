
---
--- Table:roles
---
CREATE SEQUENCE IF NOT EXISTS role_id_seq;
CREATE TABLE IF NOT EXISTS ROLES (id bigint PRIMARY KEY DEFAULT next_id_v1('role_id_seq'), name text, description text, updated_at timestamptz, created_at timestamptz DEFAULT now(), UNIQUE (name));

---
--- Table:roles:create
---
CREATE
OR REPLACE FUNCTION insert_role_v1 (name text, description text,account text, origin text) RETURNS SETOF ROLES AS $$
DECLARE inserted_roles roles;
BEGIN
   INSERT INTO
      roles(name, description)
   VALUES
      (
         name,
         description
      )
      ON CONFLICT DO NOTHING RETURNING * INTO inserted_roles;
      PERFORM insert_team_member_v1('{"kind":"TeamMember","api_version":"v1"}',json_build_object('account',account)::jsonb,json_build_object('team', inserted_roles.id::text, 'origin', origin)::jsonb);
      RETURN NEXT inserted_roles;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:roles:list_blank
---
CREATE
OR REPLACE FUNCTION get_roles_v1 () RETURNS SETOF ROLES AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      roles;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:roles:show
---
CREATE
OR REPLACE FUNCTION get_role_v1 (rid bigint) RETURNS SETOF ROLES AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      roles
   WHERE
      id = rid;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:roles:show_by_name
---
CREATE
OR REPLACE FUNCTION get_role_by_name_v1 (rname text) RETURNS SETOF ROLES AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      roles
   WHERE
      name = rname;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:permissions
---
CREATE SEQUENCE IF NOT EXISTS perm_id_seq;
CREATE TABLE IF NOT EXISTS permissions (id bigint PRIMARY KEY DEFAULT next_id_v1('perm_id_seq'), role_id bigint REFERENCES roles(id), name text, description text, updated_at timestamptz, created_at timestamptz DEFAULT now());

---
--- Table:permissions:create
---
CREATE
OR REPLACE FUNCTION insert_permission_v1 (per_role_id bigint, per_name text, per_description text) RETURNS SETOF permissions AS $$
BEGIN
   IF EXISTS
   (
      SELECT
         true
      FROM
         roles
      WHERE
         id = per_role_id
   )
THEN
   RETURN QUERY
   INSERT INTO
      permissions (role_id, name, description)
   VALUES
      (
         per_role_id, per_name, per_description
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
--- Table:permissions:show_permission_for_a_role
---
CREATE
OR REPLACE FUNCTION get_permission_by_role_v1 (perm_id bigint, rid bigint) RETURNS SETOF permissions AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      permissions
   WHERE
      role_id = rid
      AND id = perm_id
   ORDER BY
      name ASC;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:permissions:show_permissions_for_a_role
---
CREATE
OR REPLACE FUNCTION get_permissions_by_role_v1 (rid bigint) RETURNS SETOF permissions AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      permissions
   WHERE
      role_id = rid
   ORDER BY
      name ASC;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:permissions:show_permissions_for_a_role
---
CREATE
OR REPLACE FUNCTION get_permissions_by_role_name_v1 (rname text) RETURNS SETOF permissions AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      permissions
   WHERE
      role_id IN
      (
         SELECT
            id
         FROM
            roles
         WHERE
            name = rname
      )
   ORDER BY
      name ASC;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:permissions:show_permissions_for_an_user (account - email)
---
CREATE
OR REPLACE FUNCTION get_permission_by_email_v1 (r_name text) RETURNS SETOF permissions AS $$
DECLARE existing_account accounts % rowtype;
BEGIN
   SELECT
      * INTO existing_account
   FROM
      accounts
   WHERE
      email = r_name LIMIT 1;
IF FOUND
THEN
   RETURN QUERY
   SELECT
      *
   FROM
      permissions
   WHERE
      role_id IN
      (
         SELECT
            id
         FROM
            roles
         WHERE
            name = ANY((
            SELECT
               roles
            FROM
               accounts
            WHERE
               email = r_name)::text[])
      )
;
RETURN;
ELSE
   RETURN QUERY
   SELECT
      *
   FROM
      permissions
   WHERE
      role_id IN
      (
         SELECT
            id
         FROM
            roles
         WHERE
            name = ANY((
            SELECT
               roles
            FROM
               service_accounts
            WHERE
               object_meta ->> 'name' = r_name)::text[])
      )
;
RETURN;
END
IF;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;
