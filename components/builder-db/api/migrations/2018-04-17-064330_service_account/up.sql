-- Your SQL goes here

CREATE SEQUENCE IF NOT EXISTS sec_id_seq;


CREATE TABLE IF NOT EXISTS secrets (id bigint PRIMARY KEY DEFAULT next_id_v1('sec_id_seq'),
                                                                  secret_type text, DATA JSONB,
                                                                                         metadata JSONB,
                                                                                                  object_meta JSONB,
                                                                                                              type_meta JSONB,
                                                                                                                        updated_at timestamptz,
                                                                                                                        created_at timestamptz DEFAULT now());


CREATE OR REPLACE FUNCTION insert_secret_v1 (secret_type text, DATA JSONB, metadata JSONB, object_meta JSONB, type_meta JSONB) RETURNS
SETOF secrets AS $$
      BEGIN
                              RETURN QUERY INSERT INTO secrets(secret_type,data,metadata,object_meta,type_meta)
                                  VALUES (secret_type,data,metadata,object_meta,type_meta)
                                  RETURNING *;
                              RETURN;
                          END
                      $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_secret_v1 (sid bigint) RETURNS
SETOF secrets AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM secrets WHERE id = sid;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_secrets_v1() RETURNS
SETOF secrets AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM secrets;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_secrets_by_origin_v1 (origin text, name text) RETURNS
SETOF secrets AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM secrets WHERE object_meta ->> 'name' = name AND metadata ->> 'origin' = origin ;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_secrets_by_account_v1(obj_id text) RETURNS
SETOF secrets AS $$
                  BEGIN
                   RETURN QUERY SELECT * FROM secrets WHERE object_meta ->> 'account'=obj_id;
                   RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_secrets_by_origin_v1(obj_id text) RETURNS
SETOF secrets AS $$
                  BEGIN
                   RETURN QUERY SELECT * FROM secrets WHERE metadata ->> 'origin'=obj_id;
                   RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE SEQUENCE IF NOT EXISTS service_id_seq;


CREATE TABLE IF NOT EXISTS service_accounts(id bigint PRIMARY KEY DEFAULT next_id_v1('service_id_seq'),
                                                                          secrets JSONB,
                                                                                  object_meta JSONB,
                                                                                              type_meta JSONB,
                                                                                                        metadata JSONB,
                                                                                                                 ROLES text[], updated_at timestamptz,
                                                                                                                               created_at timestamptz DEFAULT now());


CREATE OR REPLACE FUNCTION insert_service_account_v1 (secrets JSONB, object_meta JSONB, type_meta JSONB, metadata JSONB, ROLES text[]) RETURNS
SETOF service_accounts AS $$
      BEGIN
           RETURN QUERY INSERT INTO service_accounts(secrets,object_meta,type_meta,metadata,roles)
               VALUES (secrets,object_meta,type_meta,metadata, roles)
               RETURNING *;
           RETURN;
      END
                      $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION update_service_account_v1 (aid bigint,sa_secrets JSONB, asm_object_meta JSONB) RETURNS
SETOF service_accounts AS $$
                      BEGIN
                          RETURN QUERY UPDATE service_accounts SET secrets=sa_secrets,object_meta = asm_object_meta,updated_at=now() WHERE id=aid
                          RETURNING *;
                          RETURN;
                      END
                   $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_service_account_v1() RETURNS
SETOF service_accounts AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM service_accounts;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_service_account_by_id_v1 (sid bigint) RETURNS
SETOF service_accounts AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM service_accounts WHERE id = sid;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_serviceaccount_by_originid_v1(ser_name text,acc_id text) RETURNS
SETOF service_accounts AS $$
                  BEGIN
                   RETURN QUERY SELECT * FROM service_accounts WHERE object_meta ->> 'name'=ser_name;
                   RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;

-- CREATE OR REPLACE FUNCTION get_permission_by_service_account_v1 (serv_name text) RETURNS
-- SETOF permissions AS $$
--               BEGIN
--                 RETURN QUERY SELECT * FROM permissions WHERE role_id IN(SELECT id FROM roles WHERE name = ANY((SELECT roles FROM service_accounts WHERE object_meta ->> 'name'=serv_name)::text[]));
--                 RETURN;
--               END
--               $$ LANGUAGE PLPGSQL STABLE;

CREATE SEQUENCE IF NOT EXISTS end_id_seq;


CREATE TABLE IF NOT EXISTS endpoints (id bigint PRIMARY KEY DEFAULT next_id_v1('end_id_seq'),
                                                                    subsets JSONB,
                                                                            object_meta JSONB,
                                                                                        type_meta JSONB,
                                                                                                  updated_at timestamptz,
                                                                                                  created_at timestamptz DEFAULT now());


CREATE OR REPLACE FUNCTION insert_endpoints_v1 (subsets JSONB, object_meta JSONB, type_meta JSONB) RETURNS
SETOF endpoints AS $$
                          BEGIN
                              RETURN QUERY INSERT INTO endpoints(subsets,object_meta,type_meta)
                                  VALUES (subsets,object_meta,type_meta )
                                  RETURNING *;
                              RETURN;
                          END
                      $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_endpoints_v1() RETURNS
SETOF endpoints AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM endpoints;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_endpoint_v1 (eid bigint) RETURNS
SETOF endpoints AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM endpoints WHERE id = eid;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_endpoints_by_account_v1(account_id text) RETURNS
SETOF endpoints AS $$
                  BEGIN
                   RETURN QUERY SELECT * FROM endpoints WHERE object_meta ->> 'account'=account_id;
                   RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_endpoints_by_assebmly_v1(pid text) RETURNS
SETOF endpoints AS $$
                  BEGIN
                   RETURN QUERY SELECT * FROM endpoints  WHERE object_meta @> json_build_object('owner_references',json_build_array(json_build_object('uid',pid)))::jsonb;
                   RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE SEQUENCE IF NOT EXISTS serv_id_seq;


CREATE TABLE IF NOT EXISTS services (id bigint PRIMARY KEY DEFAULT next_id_v1('serv_id_seq'),
                                                                   spec JSONB,
                                                                        metadata JSONB,
                                                                                 status JSONB,
                                                                                        object_meta JSONB,
                                                                                                    type_meta JSONB,
                                                                                                              updated_at timestamptz,
                                                                                                              created_at timestamptz DEFAULT now());


CREATE OR REPLACE FUNCTION insert_services_v1 (spec JSONB, metadata JSONB, status JSONB, object_meta JSONB, type_meta JSONB) RETURNS
SETOF services AS $$
                          BEGIN
                              RETURN QUERY INSERT INTO services(spec,metadata,status,object_meta,type_meta)
                                  VALUES (spec,metadata,status,object_meta,type_meta )
                                  RETURNING *;
                              RETURN;
                          END
                      $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_services_v1 (sid bigint) RETURNS
SETOF services AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM services WHERE id = sid;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_services_list_v1() RETURNS
SETOF services AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM services;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_services_by_assembly_factory_v1(pid text) RETURNS
SETOF services AS $$
                  BEGIN
                   RETURN QUERY SELECT * FROM services WHERE object_meta @> json_build_object('owner_references',json_build_array(json_build_object('uid',pid)))::jsonb;
                   RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION update_servive_by_v1 (sid bigint, spec_data JSONB, serv_metadata JSONB, status_data JSONB, object_meta_data JSONB) RETURNS
SETOF services AS $$
                      BEGIN
                          RETURN QUERY UPDATE services SET spec=spec_data,metadata=serv_metadata,status=status_data,object_meta=object_meta_data,updated_at=now() WHERE id=sid
                          RETURNING *;
                          RETURN;
                      END
                   $$ LANGUAGE PLPGSQL VOLATILE;


CREATE SEQUENCE IF NOT EXISTS set_map_id_seq;


CREATE TABLE IF NOT EXISTS settings_map (id bigint UNIQUE PRIMARY KEY DEFAULT next_id_v1('set_map_id_seq'),
                                                                              DATA JSONB,
                                                                                   metadata JSONB,
                                                                                            object_meta JSONB,
                                                                                                        type_meta JSONB,
                                                                                                                  updated_at timestamptz,
                                                                                                                  created_at timestamptz DEFAULT now());


CREATE OR REPLACE FUNCTION insert_settings_map_v1 (metadata JSONB, DATA JSONB, object_meta JSONB, type_meta JSONB) RETURNS
SETOF settings_map AS $$
                          BEGIN
                              RETURN QUERY INSERT INTO settings_map(metadata,data,object_meta,type_meta)
                                  VALUES (metadata,data,object_meta,type_meta )
                                  RETURNING *;
                              RETURN;
                          END
                      $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_settings_map_v1 (origin text, name text) RETURNS
SETOF settings_map AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM settings_map WHERE object_meta ->> 'name' = name AND metadata ->> 'origin' = origin ;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_settings_maps_v1() RETURNS
SETOF settings_map AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM settings_map;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;
