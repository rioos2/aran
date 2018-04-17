-- Your SQL goes here

CREATE SEQUENCE IF NOT EXISTS node_id_seq;


CREATE TABLE IF NOT EXISTS nodes (id bigint PRIMARY KEY DEFAULT next_id_v1('node_id_seq'),
                                                                node_ip text, spec JSONB,
                                                                                   status JSONB,
                                                                                          object_meta JSONB,
                                                                                                      type_meta JSONB,
                                                                                                                updated_at timestamptz,
                                                                                                                created_at timestamptz DEFAULT now());


CREATE OR REPLACE FUNCTION insert_node_v1 (node_ip text, spec JSONB, status JSONB, object_meta JSONB, type_meta JSONB) RETURNS
SETOF nodes AS $$
                                                                                                                                          BEGIN
                                                                                                                                              RETURN QUERY INSERT INTO nodes(node_ip,spec,status,object_meta,type_meta)
                                                                                                                                                  VALUES (node_ip,spec,status,object_meta,type_meta)
                                                                                                                                                  RETURNING *;
                                                                                                                                              RETURN;
                                                                                                                                          END
                                                                                                                                      $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_node_v1(nid bigint) RETURNS
SETOF nodes AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM nodes where id = nid;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_nodes_by_node_ip_v1(nodeip text) RETURNS
SETOF nodes AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM nodes where node_ip = nodeip;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_nodes_v1() RETURNS
SETOF nodes AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM nodes;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION set_node_status_v1 (nid bigint, node_status JSONB) RETURNS
SETOF nodes AS $$
                      BEGIN
                          RETURN QUERY UPDATE nodes SET status=node_status, updated_at=now() WHERE id=nid
                          RETURNING *;
                          RETURN;
                      END
                   $$ LANGUAGE PLPGSQL VOLATILE;
