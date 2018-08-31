---
--- Your SQL goes here

CREATE SEQUENCE IF NOT EXISTS storage_id_seq;


CREATE TABLE IF NOT EXISTS storages (id bigint UNIQUE PRIMARY KEY DEFAULT next_id_v1('storage_id_seq'),
                                                                          host_ip text, storage_type text, PARAMETERS JSONB,
                                                                                                                      storage_info JSONB,
                                                                                                                                   node_info JSONB,
                                                                                                                                             status JSONB,
                                                                                                                                                    object_meta JSONB,
                                                                                                                                                                type_meta JSONB,
                                                                                                                                                                          updated_at timestamptz,
                                                                                                                                                                          created_at timestamptz DEFAULT now());


CREATE OR REPLACE FUNCTION insert_storage_v1 (host_ip text, storage_type text, PARAMETERS JSONB, storage_info JSONB, node_info JSONB, status JSONB, object_meta JSONB, type_meta JSONB) RETURNS
SETOF storages AS $$
                          BEGIN
                              RETURN QUERY INSERT INTO storages(host_ip,storage_type,parameters,storage_info,node_info,status,object_meta,type_meta)
                                  VALUES (host_ip,storage_type,parameters,storage_info,node_info,status,object_meta,type_meta)
                                  RETURNING *;
                              RETURN;
                          END
                      $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_storages_v1() RETURNS
SETOF storages AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM storages;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_storages_by_ip_v1 (hostip text) RETURNS
SETOF storages AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM storages WHERE host_ip = hostip;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_storage_v1 (sid bigint) RETURNS
SETOF storages AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM storages WHERE id = sid;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION set_storage_status_v1 (sid bigint, storage_status JSONB) RETURNS
SETOF storages AS $$
                      BEGIN
                          RETURN QUERY UPDATE storages SET status=storage_status, updated_at=now() WHERE id=sid
                          RETURNING *;
                          RETURN;
                      END
                   $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION update_storage_v1(sid bigint, s_host_ip text, s_storage_type text, s_parameters JSONB, s_storage_info JSONB, s_node_info JSONB, s_status JSONB, s_object_meta JSONB) RETURNS
SETOF storages AS $$
                      BEGIN
                          RETURN QUERY UPDATE storages SET host_ip=s_host_ip,storage_type=s_storage_type,parameters=s_parameters,storage_info=s_storage_info,node_info=s_node_info,status = s_status,object_meta=s_object_meta,updated_at=now() WHERE id=sid
                          RETURNING *;
                          RETURN;
                      END
                   $$ LANGUAGE PLPGSQL VOLATILE;


CREATE SEQUENCE IF NOT EXISTS dc_id_seq;


CREATE TABLE IF NOT EXISTS data_centers (id bigint PRIMARY KEY DEFAULT next_id_v1('dc_id_seq'),
                                                                       nodes text[], networks text[], enabled bool,
                                         STORAGE text, advanced_settings JSONB,
                                                                         flag text, currency text, status JSONB,
                                                                                                          object_meta JSONB,
                                                                                                                      type_meta JSONB,
                                                                                                                                updated_at timestamptz,
                                                                                                                                created_at timestamptz DEFAULT now());


CREATE OR REPLACE FUNCTION insert_dc_v1 (nodes text[], networks text[], enabled bool,
                                         STORAGE text, advanced_settings JSONB, flag text, currency text, status JSONB, object_meta JSONB, type_meta JSONB) RETURNS
SETOF data_centers AS $$
                          BEGIN
                              RETURN QUERY INSERT INTO data_centers(nodes,networks,enabled,storage,advanced_settings,flag,currency,status,object_meta,type_meta)
                                  VALUES (nodes,networks,enabled,storage,advanced_settings,flag,currency,status,object_meta,type_meta)
                                  RETURNING *;
                              RETURN;
                          END
                      $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_data_centers_v1() RETURNS
SETOF data_centers AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM data_centers;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_data_center_v1(did bigint) RETURNS
SETOF data_centers AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM data_centers WHERE id = did;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;

CREATE OR REPLACE FUNCTION get_data_center_by_name_v1(dc_name text) RETURNS
SETOF data_centers AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM data_centers WHERE  object_meta ->> 'name' = dc_name;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION update_datacenter_by_v1(dc_id bigint, dc_nodes text[], dc_networks text[], dc_enabled bool, dc_storage text, dc_advanced_settings JSONB, dc_flag text, dc_currency text, dc_status JSONB, dc_object_meta JSONB) RETURNS
SETOF data_centers AS $$
                      BEGIN
                          RETURN QUERY UPDATE data_centers SET nodes=dc_nodes,networks=dc_networks,enabled=dc_enabled,storage=dc_storage,advanced_settings= dc_advanced_settings,flag=dc_flag,currency=dc_currency,status=dc_status,object_meta=dc_object_meta,updated_at=now() WHERE id=dc_id
                          RETURNING *;
                          RETURN;
                      END
                   $$ LANGUAGE PLPGSQL VOLATILE;


CREATE SEQUENCE IF NOT EXISTS storages_pool_id_seq;


CREATE TABLE IF NOT EXISTS storages_pool (id bigint PRIMARY KEY DEFAULT next_id_v1('storages_pool_id_seq'),
                                                                        connector_id bigint REFERENCES storages(id),
                                                                                                       PARAMETERS JSONB,
                                                                                                                  remote_storage_disks JSONB,
                                                                                                                                       storage_info JSONB,
                                                                                                                                                    status JSONB,
                                                                                                                                                           object_meta JSONB,
                                                                                                                                                                       type_meta JSONB,
                                                                                                                                                                                 updated_at timestamptz,
                                                                                                                                                                                 created_at timestamptz DEFAULT now());


CREATE OR REPLACE FUNCTION insert_storage_pool_v1 (connector_id bigint, PARAMETERS JSONB, remote_storage_disks JSONB, storage_info JSONB, status JSONB, object_meta JSONB, type_meta JSONB) RETURNS
SETOF storages_pool AS $$
                          BEGIN
                              RETURN QUERY INSERT INTO storages_pool(connector_id,parameters,remote_storage_disks,storage_info,status, object_meta,type_meta)
                                  VALUES (connector_id,parameters,remote_storage_disks,storage_info,status,object_meta,type_meta)
                                  RETURNING *;
                              RETURN;
                          END
                      $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_storage_pool_v1 (sid bigint) RETURNS
SETOF storages_pool AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM storages_pool WHERE connector_id = sid;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_storage_pool_by_id_v1 (sid bigint) RETURNS
SETOF storages_pool AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM storages_pool WHERE id = sid;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_storage_pool_all_v1() RETURNS
SETOF storages_pool AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM storages_pool;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION set_storage_pool_status_v1 (sid bigint, sp_status JSONB) RETURNS
SETOF storages_pool AS $$
                      BEGIN
                          RETURN QUERY UPDATE storages_pool SET status=sp_status, updated_at=now() WHERE id=sid
                          RETURNING *;
                          RETURN;
                      END
                   $$ LANGUAGE PLPGSQL VOLATILE;
