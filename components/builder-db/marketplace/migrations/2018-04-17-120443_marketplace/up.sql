-- Your SQL goes here

CREATE SEQUENCE IF NOT EXISTS market_id_seq;


CREATE TABLE IF NOT EXISTS marketplaces (id bigint PRIMARY KEY DEFAULT next_id_v1('market_id_seq'),
                                                                       type_meta JSONB,
                                                                                 object_meta JSONB,
                                                                                             category text, VERSION text, CHARACTERISTICS JSONB,
                                                                                                                                          icon text, description text, ports JSONB,
                                                                                                                                                                             envs JSONB,
                                                                                                                                                                                  lifecycle JSONB,
                                                                                                                                                                                            status JSONB,
                                                                                                                                                                                                   metadata JSONB,
                                                                                                                                                                                                            updated_at timestamptz,
                                                                                                                                                                                                            created_at timestamptz DEFAULT now());


CREATE OR REPLACE FUNCTION insert_marketplace_v1 (type_meta JSONB, object_meta JSONB, category text, VERSION text, CHARACTERISTICS JSONB, icon text, description text, ports JSONB, envs JSONB, lifecycle JSONB, status JSONB, metadata JSONB) RETURNS
SETOF marketplaces AS $$
                                                                                                                                                                                                                                  BEGIN
                                                                                                                                                                                                                                      RETURN QUERY INSERT INTO marketplaces(type_meta, object_meta, category,version, characteristics, icon,description,ports,envs,lifecycle,status,metadata)
                                                                                                                                                                                                                                          VALUES (type_meta, object_meta, category,version, characteristics, icon,description,ports,envs,lifecycle,status,metadata)
                                                                                                                                                                                                                                          RETURNING *;
                                                                                                                                                                                                                                      RETURN;
                                                                                                                                                                                                                                  END
                                                                                                                                                                                                                              $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_marketplace_v1(mid bigint) RETURNS
SETOF marketplaces AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM marketplaces WHERE id=mid;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_marketplaces_v1() RETURNS
SETOF marketplaces AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM marketplaces;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;
