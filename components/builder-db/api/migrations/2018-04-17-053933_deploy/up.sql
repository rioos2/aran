-- Your SQL goes here

CREATE SEQUENCE IF NOT EXISTS asm_id_seq;


CREATE TABLE IF NOT EXISTS assemblys (id bigint PRIMARY KEY DEFAULT next_id_v1('asm_id_seq'),
                                                                    type_meta JSONB,
                                                                              object_meta JSONB,
                                                                                          selector text[], status JSONB,
                                                                                                                  metadata JSONB,
                                                                                                                           updated_at timestamptz,
                                                                                                                           created_at timestamptz DEFAULT now());


CREATE OR REPLACE FUNCTION insert_assembly_v1 (type_meta JSONB, object_meta JSONB, selector text[], status JSONB, metadata JSONB) RETURNS
SETOF assemblys AS $$
                                                                                                                                                     BEGIN
                                                                                                                                                         RETURN QUERY INSERT INTO assemblys(type_meta,object_meta,selector,status,metadata)
                                                                                                                                                             VALUES (type_meta,object_meta,selector,status,metadata)
                                                                                                                                                             RETURNING *;
                                                                                                                                                         RETURN;
                                                                                                                                                     END
                                                                                                                                                 $$ LANGUAGE PLPGSQL VOLATILE;


CREATE SEQUENCE IF NOT EXISTS plan_id_seq;


CREATE TABLE IF NOT EXISTS plan_factory (id bigint PRIMARY KEY DEFAULT next_id_v1('plan_id_seq'),
                                                                       type_meta JSONB,
                                                                                 object_meta JSONB,
                                                                                             category text, VERSION text, CHARACTERISTICS JSONB,
                                                                                                                                          icon text, description text, ports JSONB,
                                                                                                                                                                             envs JSONB,
                                                                                                                                                                                  lifecycle JSONB,
                                                                                                                                                                                            status JSONB,
                                                                                                                                                                                                   updated_at timestamptz,
                                                                                                                                                                                                   created_at timestamptz DEFAULT now());


CREATE OR REPLACE FUNCTION get_assembly_v1 (aid bigint) RETURNS
SETOF assemblys AS $$
BEGIN
RETURN QUERY SELECT * FROM assemblys WHERE id = aid;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_assemblys_v1() RETURNS
SETOF assemblys AS $$
 BEGIN
 RETURN QUERY SELECT * FROM assemblys;
 RETURN;
 END
 $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_assemblys_by_parentid_v1 (pid text) RETURNS
SETOF assemblys AS $$
 BEGIN
 RETURN QUERY SELECT * FROM assemblys  WHERE object_meta @> json_build_object('owner_references',json_build_array(json_build_object('uid',pid)))::jsonb;
 RETURN;
 END
 $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION update_assembly_v1 (aid bigint,asm_selector text[],asm_status JSONB, asm_object_meta JSONB, asm_metadata JSONB) RETURNS
SETOF assemblys AS $$
  BEGIN
  RETURN QUERY UPDATE assemblys SET selector=asm_selector,status=asm_status,object_meta = asm_object_meta,metadata=asm_metadata,updated_at=now() WHERE id=aid
  RETURNING *;
  RETURN;
  END
  $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION set_assembly_status_v1 (aid bigint, asm_status JSONB) RETURNS
SETOF assemblys AS $$
  BEGIN
  RETURN QUERY UPDATE assemblys SET status=asm_status, updated_at=now() WHERE id=aid
  RETURNING *;
  RETURN;
  END
  $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_assemblys_by_account_v1 (account_id text) RETURNS
SETOF assemblys AS $$
 BEGIN
 RETURN QUERY SELECT * FROM assemblys WHERE object_meta ->> 'account' = account_id;
 RETURN;
 END
 $$ LANGUAGE PLPGSQL STABLE;


CREATE SEQUENCE IF NOT EXISTS asm_fact_id_seq;


CREATE TABLE IF NOT EXISTS assembly_factory (id bigint PRIMARY KEY DEFAULT next_id_v1('asm_fact_id_seq'),
                                                                           object_meta JSONB,
                                                                                       type_meta JSONB,
                                                                                                 replicas smallint, resources JSONB,
                                                                                                                              metadata JSONB,
                                                                                                                                       status JSONB,
                                                                                                                                              secret JSONB,
                                                                                                                                                     PLAN bigint REFERENCES plan_factory(id),
                                                                                                                                                                            spec JSONB,
                                                                                                                                                                                 updated_at timestamptz,
                                                                                                                                                                                 created_at timestamptz DEFAULT now());


CREATE OR REPLACE FUNCTION insert_assembly_factory_v1 (object_meta JSONB, type_meta JSONB, replicas smallint, resources JSONB, metadata JSONB, status JSONB, secret JSONB, PLAN bigint, spec JSONB) RETURNS
SETOF assembly_factory AS $$
BEGIN
RETURN QUERY INSERT INTO assembly_factory(object_meta,type_meta,replicas,resources,metadata,status,secret,plan,spec)
VALUES (object_meta,type_meta,replicas,resources,metadata,status,secret,plan,spec)
RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_assembly_factory_v1 (aid bigint) RETURNS
SETOF assembly_factory AS $$
BEGIN
RETURN QUERY SELECT * FROM assembly_factory WHERE id = aid;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_assemblys_factory_v1() RETURNS
SETOF assembly_factory AS $$
BEGIN
RETURN QUERY SELECT * FROM assembly_factory;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION set_assembly_factorys_status_v1 (aid bigint, asm_fac_status JSONB) RETURNS
SETOF assembly_factory AS $$
BEGIN
RETURN QUERY UPDATE assembly_factory SET status=asm_fac_status, updated_at=now() WHERE id=aid
RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_assembly_factory_by_account_v1 (account_id text) RETURNS
SETOF assembly_factory AS $$
BEGIN
RETURN QUERY SELECT * FROM assembly_factory WHERE object_meta ->> 'account' = account_id;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;
