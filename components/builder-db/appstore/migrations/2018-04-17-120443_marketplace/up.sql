-- Your SQL goes here

CREATE SEQUENCE IF NOT EXISTS market_id_seq;


CREATE TABLE IF NOT EXISTS marketplaces (id bigint PRIMARY KEY DEFAULT next_id_v1('market_id_seq'), type_meta JSONB, object_meta JSONB, plans JSONB, category text, VERSION text, icon text, description text, status JSONB, created_at timestamptz DEFAULT now());


CREATE OR REPLACE FUNCTION insert_marketplace_v1 (type_meta JSONB, object_meta JSONB, plans JSONB,  category text, VERSION text, icon text, description text, status JSONB) RETURNS
SETOF marketplaces AS $$
                  BEGIN
                    RETURN QUERY INSERT INTO marketplaces(type_meta, object_meta, plans, category, version, icon, description, status)
                      VALUES (type_meta, object_meta, plans, category, version, icon, description, status)
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
