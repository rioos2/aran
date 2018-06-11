---
--- Table:settings_map
---
CREATE SEQUENCE IF NOT EXISTS set_map_id_seq;
CREATE TABLE IF NOT EXISTS settings_map (id bigint UNIQUE PRIMARY KEY DEFAULT next_id_v1('set_map_id_seq'), DATA JSONB, metadata JSONB, object_meta JSONB, type_meta JSONB, updated_at timestamptz, created_at timestamptz DEFAULT now());

---
--- Table:settings_map:create
---
CREATE 
OR REPLACE FUNCTION insert_settings_map_v1 (metadata JSONB, DATA JSONB, object_meta JSONB, type_meta JSONB) RETURNS SETOF settings_map AS $$ 
BEGIN
   RETURN QUERY 
   INSERT INTO
      settings_map(metadata, data, object_meta, type_meta) 
   VALUES
      (
         metadata,
         data,
         object_meta,
         type_meta 
      )
      RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:settings_map:show_by_name_origin
---
CREATE 
OR REPLACE FUNCTION get_settings_map_v1 (origin text, name text) RETURNS SETOF settings_map AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      * 
   FROM
      settings_map 
   WHERE
      object_meta ->> 'name' = name 
      AND metadata ->> 'origin' = origin ;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:settings_map:list_blank
---
CREATE 
OR REPLACE FUNCTION get_settings_maps_v1() RETURNS SETOF settings_map AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      * 
   FROM
      settings_map;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;