-- You---
--- Your SQL goes here 

CREATE SEQUENCE IF NOT EXISTS ingress_id_seq;


CREATE TABLE IF NOT EXISTS ingress (id bigint PRIMARY KEY DEFAULT next_id_v1('ingress_id_seq'),
                                                                  status JSONB, object_meta JSONB, type_meta JSONB, spec JSONB,
                                                                                                                                       updated_at timestamptz,
                                                                                                                                       created_at timestamptz DEFAULT now());


CREATE OR REPLACE FUNCTION insert_ingress_v1 (status JSONB, object_meta JSONB, type_meta JSONB, spec JSONB) RETURNS
SETOF ingress AS $$
                          BEGIN
                              RETURN QUERY INSERT INTO ingress(status,object_meta, type_meta, spec)
                                  VALUES (status, object_meta, type_meta, spec)
                                  RETURNING *;
                              RETURN;
                          END
                      $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_ingress_v1(ing_id bigint) RETURNS
SETOF ingress AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM ingress where id = ing_id;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION set_ingress_status_v1 (ing_id bigint, ingress_status JSONB) RETURNS
SETOF ingress AS $$
                      BEGIN
                          RETURN QUERY UPDATE ingress SET status=ingress_status, updated_at=now() WHERE id=ing_id
                          RETURNING *;
                          RETURN;
                      END
                   $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_ingress_by_assembly_factory_v1(aid text) RETURNS
SETOF ingress AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM ingress where object_meta @> json_build_object('owner_references',json_build_array(json_build_object('uid',aid)))::jsonb;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION update_ingress_v1 (iid bigint,ing_status JSONB, ing_object_meta JSONB, ing_spec JSONB) RETURNS
SETOF ingress AS $$
                      BEGIN
                          RETURN QUERY UPDATE ingress SET status=ing_status,object_meta = ing_object_meta, spec = ing_spec, updated_at=now() WHERE id=iid
                          RETURNING *;
                          RETURN;
                      END
                   $$ LANGUAGE PLPGSQL VOLATILE;
