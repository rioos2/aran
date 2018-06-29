---
--- Table:licenses
---

CREATE SEQUENCE IF NOT EXISTS license_id_seq;


CREATE TABLE IF NOT EXISTS LICENSES (id bigint PRIMARY KEY DEFAULT next_id_v1('license_id_seq'),
                                                                   name text, status text, updated_at timestamptz,
                                                                                           created_at timestamptz DEFAULT now(),
                                                                                                                          UNIQUE (name));

---
--- Table:licenses:create
---

CREATE FUNCTION insert_or_update_license_v1(lname text, lstatus text) RETURNS
SETOF LICENSES AS $$
DECLARE this_license licenses % rowtype;
BEGIN
    SELECT
       *
    FROM
       licenses
    WHERE
       licenses.name = lname LIMIT 1 INTO this_license;
       IF FOUND THEN
       RETURN QUERY
       UPDATE
          LICENSES
       SET
          status = lstatus,
          updated_at = now()
       WHERE
          name = lname RETURNING *;
          RETURN;
          ELSE
          RETURN QUERY
        INSERT INTO LICENSES(name,status) VALUES (lname, lstatus) ON CONFLICT DO NOTHING RETURNING *;
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
      name = lname;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;
