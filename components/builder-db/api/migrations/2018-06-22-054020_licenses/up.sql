---
--- Table:licenses
---

CREATE SEQUENCE IF NOT EXISTS license_id_seq;


CREATE TABLE IF NOT EXISTS LICENSES (id bigint PRIMARY KEY DEFAULT next_id_v1('license_id_seq'),
                                                                   object_meta JSONB,
                                                                               type_meta JSONB,
                                                                                         status text, product text,activation_code text,expired text, updated_at timestamptz,
                                                                                                                                                      created_at timestamptz DEFAULT now());

---
--- Table:licenses:create/update
---

CREATE FUNCTION insert_or_update_license_v1( lobject_meta JSONB, ltype_meta JSONB, lstatus text, lproduct text,lactivation_code text,lexpired text )RETURNS
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
         activation_code = lactivation_code,
         expired = lexpired,
         updated_at = now()
      WHERE
         object_meta ->> 'name' = lobject_meta ->> 'name' RETURNING *;
         RETURN;
         ELSE
         RETURN QUERY
         INSERT INTO LICENSES(object_meta,type_meta, status, product,activation_code, expired )
           VALUES(lobject_meta, ltype_meta, lstatus, lproduct,lactivation_code,lexpired )ON CONFLICT DO NOTHING RETURNING *;
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
