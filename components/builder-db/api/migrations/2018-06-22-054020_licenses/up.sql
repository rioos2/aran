---
--- Table:licenses
---
CREATE SEQUENCE IF NOT EXISTS license_id_seq;
CREATE TABLE IF NOT EXISTS LICENSES (id bigint PRIMARY KEY DEFAULT next_id_v1('license_id_seq'), name text, status text, updated_at timestamptz, created_at timestamptz DEFAULT now(), UNIQUE (name));

---
--- Table:licenses:create
---
CREATE FUNCTION insert_or_update_license_v1(lname text, lstatus text) RETURNS SETOF LICENSES AS $$
BEGIN
    LOOP
        -- first try to update the name
        -- note that "name" must be unique
        UPDATE LICENSES SET status = lstatus WHERE name = lname;
        IF found THEN
            RETURN;
        END IF;
        -- not there, so try to insert the name
        -- if someone else inserts the same name concurrently,
        -- we could get a unique-key failure
        BEGIN
            INSERT INTO LICENSES(name,status) VALUES (lname, lstatus);
            RETURN;
        EXCEPTION WHEN unique_violation THEN
            -- do nothing, and loop to try the UPDATE again
        END;
    END LOOP;
END;
$$
LANGUAGE plpgsql;

---
--- Table:licenses:show
---
CREATE 
OR REPLACE FUNCTION get_license_v1(name text) RETURNS SETOF licenses AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      * 
   FROM
      licenses 
   where
      name = name;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;
