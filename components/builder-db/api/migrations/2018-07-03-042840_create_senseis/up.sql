---
---
--- Table:senseis
---
CREATE SEQUENCE IF NOT EXISTS senseis_id_seq;
CREATE TABLE IF NOT EXISTS senseis (id bigint PRIMARY KEY DEFAULT next_id_v1('senseis_id_seq'), node_ip text, spec JSONB, status JSONB, object_meta JSONB, type_meta JSONB, metadata JSONB, updated_at timestamptz, created_at timestamptz DEFAULT now());

  CREATE FUNCTION insert_or_update_senseis_v1(node_ip text, spec JSONB, sen_status JSONB, sen_object_meta JSONB, type_meta JSONB, metadata JSONB) RETURNS SETOF senseis AS $$
  DECLARE this_senseis senseis % rowtype;
  BEGIN
     SELECT
        *
     FROM
        senseis
     WHERE
        senseis.object_meta ->> 'name' = sen_object_meta ->> 'name' LIMIT 1 INTO this_senseis;
        IF FOUND THEN
        RETURN QUERY
        UPDATE
           senseis
        SET
           status = sen_status,
           updated_at = now()
        WHERE
           object_meta ->> 'name' = sen_object_meta ->> 'name' RETURNING *;
           RETURN;
           ELSE
           RETURN QUERY
           INSERT INTO senseis(node_ip, spec, status, object_meta, type_meta,metadata)VALUES(node_ip, spec, sen_status, sen_object_meta, type_meta, metadata)ON CONFLICT DO NOTHING RETURNING *;
        RETURN;
        END
        IF;
        RETURN;
  END
  $$ LANGUAGE PLPGSQL;
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
