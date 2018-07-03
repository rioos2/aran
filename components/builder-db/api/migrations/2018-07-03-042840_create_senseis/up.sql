---
---
--- Table:senseis
---
CREATE SEQUENCE IF NOT EXISTS senseis_id_seq;
CREATE TABLE IF NOT EXISTS senseis (id bigint PRIMARY KEY DEFAULT next_id_v1('senseis_id_seq'), node_ip text, spec JSONB, status JSONB, object_meta JSONB, type_meta JSONB, metadata JSONB, updated_at timestamptz, created_at timestamptz DEFAULT now());

---
--- Table:senseis:create
---
CREATE
OR REPLACE FUNCTION insert_senseis_v1 (node_ip text, spec JSONB, status JSONB, object_meta JSONB, type_meta JSONB, metadata JSONB) RETURNS SETOF senseis AS $$
BEGIN
   RETURN QUERY
   INSERT INTO
      senseis(node_ip, spec, status, object_meta, type_meta,metadata)
   VALUES
      (
         node_ip, spec, status, object_meta, type_meta, metadata
      )
      RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:senseis:show
---
CREATE
OR REPLACE FUNCTION get_sensei_v1(sid bigint) RETURNS SETOF senseis AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      senseis
   where
      id = sid;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:senseis:list_blank
---
CREATE
OR REPLACE FUNCTION get_senseis_v1() RETURNS SETOF senseis AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      senseis;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;
