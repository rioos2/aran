---
--- Table:licenses
---

CREATE SEQUENCE IF NOT EXISTS license_id_seq;


CREATE TABLE IF NOT EXISTS LICENSES (id bigint PRIMARY KEY DEFAULT next_id_v1('license_id_seq'),
                                                                   object_meta JSONB,
                                                                               type_meta JSONB,
                                                                                         status text, product text, license_id text,password text,expired text, product_options JSONB,
                                                                                                                                                                                updated_at timestamptz,
                                                                                                                                                                                created_at timestamptz DEFAULT now());

---
--- Table:licenses:create/update
---

CREATE FUNCTION insert_or_update_license_v1(lobject_meta JSONB, ltype_meta JSONB, lstatus text, lproduct text,lexpired text, lproduct_options JSONB)RETURNS
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
         product_options = lproduct_options,
         updated_at = now()
      WHERE
         object_meta ->> 'name' = lobject_meta ->> 'name' RETURNING *;
         RETURN;
         ELSE
         RETURN QUERY
         INSERT INTO LICENSES(object_meta,type_meta, status, product,expired, product_options )
           VALUES(lobject_meta, ltype_meta, lstatus, lproduct,lexpired ,lproduct_options)ON CONFLICT DO NOTHING RETURNING *;
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

CREATE OR REPLACE FUNCTION update_license_product_options_v1 (lid bigint, li_product_options JSONB) RETURNS
SETOF licenses AS $$
                      BEGIN
                          RETURN QUERY UPDATE licenses SET product_options=li_product_options, updated_at=now() WHERE id=lid
                          RETURNING *;
                          RETURN;
                      END
                   $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION update_license_status_v1 (llicense_id text,lpassword text,lstatus text) RETURNS
SETOF licenses AS $$
                                         BEGIN
                                             RETURN QUERY UPDATE licenses SET license_id  = llicense_id,password=lpassword,status=lstatus,updated_at=now()
                                             RETURNING *;
                                             RETURN;
                                         END
                                      $$ LANGUAGE PLPGSQL VOLATILE;
