---
--- Table:licenses
---

CREATE SEQUENCE IF NOT EXISTS license_id_seq;


CREATE TABLE IF NOT EXISTS LICENSES (id bigint PRIMARY KEY DEFAULT next_id_v1('license_id_seq'),
                                                                   object_meta JSONB,
                                                                               type_meta JSONB,
                                                                                         status text, license_id text,password text,expired text, activation JSONB,user_activation bool, provider text, error text,
                                                                                                                                                                                updated_at timestamptz,
                                                                                                                                                                                created_at timestamptz DEFAULT now());

---
--- Table:licenses:create/update
---

CREATE FUNCTION insert_or_update_license_v1(lobject_meta JSONB, ltype_meta JSONB, lstatus text, lexpired text, lactivation JSONB,luser_activation bool,lprovider text,lerror text)RETURNS
SETOF LICENSES AS $$
DECLARE this_license LICENSES % rowtype;
BEGIN
   SELECT
      *
   FROM
      LICENSES
   WHERE
      LICENSES.object_meta ->> 'name' = lobject_meta ->> 'name' LIMIT 1 INTO this_license;
      IF FOUND THEN
      RETURN QUERY
      UPDATE
         LICENSES
      SET
         status = lstatus,
         expired = lexpired,
         activation = lactivation,
         updated_at = now()
      WHERE
         object_meta ->> 'name' = lobject_meta ->> 'name' RETURNING *;
         RETURN;
         ELSE
         RETURN QUERY
         INSERT INTO LICENSES(object_meta,type_meta, status,expired, activation, user_activation,provider,error)
           VALUES(lobject_meta, ltype_meta, lstatus,lexpired ,lactivation, luser_activation,lprovider,lerror)ON CONFLICT DO NOTHING RETURNING *;
      RETURN;
      END
      IF;
      RETURN;
END
$$ LANGUAGE PLPGSQL;

---
--- Table:licenses:show
---

CREATE
OR REPLACE FUNCTION get_license_v1(lname text) RETURNS
SETOF licenses AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      licenses
   where
      object_meta ->> 'name' = lname;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:licenses:list
---

CREATE
OR REPLACE FUNCTION get_license_list_by_v1() RETURNS
SETOF licenses AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      licenses;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:licenses:update
---

CREATE OR REPLACE FUNCTION update_license_v1 (lname text, li_activation JSONB,llicense_id text,lpassword text,lexpired text,lstatus text,lerror text) RETURNS
SETOF licenses AS $$
                      BEGIN
                          RETURN QUERY UPDATE licenses SET
                          license_id  = llicense_id,
                          password=lpassword,
                          status = lstatus,
                          expired = lexpired,
                          activation=li_activation,
                          error=lerror,
                          updated_at=now() WHERE object_meta ->> 'name' = lname
                          RETURNING *;
                          RETURN;
                      END
                   $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION update_activation_complete_v1 (lid bigint,luser_activation bool) RETURNS
SETOF licenses AS $$
                                         BEGIN
                                             RETURN QUERY UPDATE licenses SET user_activation=luser_activation,updated_at=now() WHERE id = lid
                                             RETURNING *;
                                             RETURN;
                                         END
                                      $$ LANGUAGE PLPGSQL VOLATILE;

CREATE OR REPLACE FUNCTION update_error_v1 (lname text,lerror text) RETURNS
SETOF licenses AS $$
                                         BEGIN
                                             RETURN QUERY UPDATE licenses SET error=lerror,updated_at=now() WHERE  object_meta ->> 'name' = lname
                                             RETURNING *;
                                             RETURN;
                                         END
                                      $$ LANGUAGE PLPGSQL VOLATILE;
