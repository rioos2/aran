-- Your SQL goes here

CREATE SEQUENCE IF NOT EXISTS package_id_seq;


CREATE TABLE IF NOT EXISTS packages (id bigint PRIMARY KEY DEFAULT next_id_v1('package_id_seq'),
                                                                   version_number text, extension text, object_meta JSONB,
                                                                                                                    type_meta JSONB,
                                                                                                                              updated_at timestamptz,
                                                                                                                              created_at timestamptz DEFAULT now());


CREATE OR REPLACE FUNCTION insert_package_v1 (type_meta JSONB, object_meta JSONB, version_number text, extension text) RETURNS
SETOF packages AS $$
                                                                                                                                                    BEGIN
                                                                                                                                                        RETURN QUERY INSERT INTO packages(type_meta, object_meta, version_number,extension)
                                                                                                                                                            VALUES (type_meta, object_meta, version_number,extension)
                                                                                                                                                            RETURNING *;
                                                                                                                                                        RETURN;
                                                                                                                                                    END
                                                                                                                                                $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_package_v1(pid bigint) RETURNS
SETOF packages AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM packages WHERE id=pid;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;
