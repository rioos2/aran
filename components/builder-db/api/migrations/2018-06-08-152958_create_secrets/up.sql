---
--- Table:secrets
---

CREATE SEQUENCE IF NOT EXISTS sec_id_seq;


CREATE TABLE IF NOT EXISTS secrets (id bigint PRIMARY KEY DEFAULT next_id_v1('sec_id_seq'),
                                                                  secret_type text, DATA JSONB,
                                                                                         metadata JSONB,
                                                                                                  object_meta JSONB,
                                                                                                              type_meta JSONB,
                                                                                                                        updated_at timestamptz,
                                                                                                                        created_at timestamptz DEFAULT now());

---
--- Table:secrets:create
---

CREATE
OR REPLACE FUNCTION insert_secret_v1 (secret_type text, DATA JSONB, metadata JSONB, object_meta JSONB, type_meta JSONB) RETURNS
SETOF secrets AS $$
BEGIN
   RETURN QUERY
   INSERT INTO
      secrets(secret_type, data, metadata, object_meta, type_meta)
   VALUES
      (
         secret_type, data, metadata, object_meta, type_meta
      )
      RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:secrets:show
---

CREATE
OR REPLACE FUNCTION get_secret_v1 (sid bigint) RETURNS
SETOF secrets AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      secrets
   WHERE
      id = sid;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:secrets:list_blank
---

CREATE
OR REPLACE FUNCTION get_secrets_v1() RETURNS
SETOF secrets AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      secrets;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:secrets:show_by_origin & name
--- What the heck will be in `name`
---

CREATE
OR REPLACE FUNCTION get_secrets_by_origin_v1 (origin text, name text) RETURNS
SETOF secrets AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      secrets
   WHERE
      object_meta ->> 'name' = name
      AND metadata ->> 'origin' = origin ;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:secrets:show_by_account
---

CREATE
OR REPLACE FUNCTION get_secrets_by_account_v1(obj_id text) RETURNS
SETOF secrets AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      secrets
   WHERE
      object_meta ->> 'account' = obj_id;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:secrets:show_by_origin
---

CREATE
OR REPLACE FUNCTION get_secrets_by_origin_id_v1(obj_id text) RETURNS
SETOF secrets AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      secrets
   WHERE
      metadata ->> 'origin' = obj_id;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;
