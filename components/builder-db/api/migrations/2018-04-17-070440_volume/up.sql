---
--- Your SQL goes here

CREATE SEQUENCE IF NOT EXISTS volume_id_seq;


CREATE TABLE IF NOT EXISTS volumes (id bigint PRIMARY KEY DEFAULT next_id_v1('volume_id_seq'),
                                                                  mount_path text, allocated text, status JSONB,
                                                                                                          object_meta JSONB,
                                                                                                                      type_meta JSONB,
                                                                                                                                SOURCE JSONB,
                                                                                                                                       updated_at timestamptz,
                                                                                                                                       created_at timestamptz DEFAULT now());


CREATE OR REPLACE FUNCTION insert_volume_v1 (mount_path text, allocated text, status JSONB, object_meta JSONB, type_meta JSONB, SOURCE JSONB) RETURNS
SETOF volumes AS $$
                          BEGIN
                              RETURN QUERY INSERT INTO volumes(mount_path,allocated,status,object_meta, type_meta, source)
                                  VALUES (mount_path,allocated,status, object_meta, type_meta, source)
                                  RETURNING *;
                              RETURN;
                          END
                      $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_volume_v1(vid bigint) RETURNS
SETOF volumes AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM volumes where id = vid;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION set_volume_status_v1 (vid bigint, volume_status JSONB) RETURNS
SETOF volumes AS $$
                      BEGIN
                          RETURN QUERY UPDATE volumes SET status=volume_status, updated_at=now() WHERE id=vid
                          RETURNING *;
                          RETURN;
                      END
                   $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_volumes_by_assembly_v1(aid text) RETURNS
SETOF volumes AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM volumes where object_meta @> json_build_object('owner_references',json_build_array(json_build_object('uid',aid)))::jsonb;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION update_volume_v1 (vid bigint,vol_mount_path text,vol_allocated text,vol_status JSONB, vol_object_meta JSONB, vol_source JSONB) RETURNS
SETOF volumes AS $$
                      BEGIN
                          RETURN QUERY UPDATE volumes SET mount_path=vol_mount_path,allocated=vol_allocated,status=vol_status,object_meta = vol_object_meta, source = vol_source, updated_at=now() WHERE id=vid
                          RETURNING *;
                          RETURN;
                      END
                   $$ LANGUAGE PLPGSQL VOLATILE;
