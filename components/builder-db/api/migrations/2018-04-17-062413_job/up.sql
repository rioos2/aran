-- Your SQL goes here

CREATE SEQUENCE IF NOT EXISTS job_id_seq;


CREATE TABLE IF NOT EXISTS jobs (id bigint PRIMARY KEY DEFAULT next_id_v1('job_id_seq'),
                                                               spec JSONB,
                                                                    status JSONB,
                                                                           object_meta JSONB,
                                                                                       type_meta JSONB,
                                                                                                 updated_at timestamptz,
                                                                                                 created_at timestamptz DEFAULT now());
CREATE OR REPLACE FUNCTION insert_jobs_v1 (spec JSONB, status JSONB, object_meta JSONB, type_meta JSONB) RETURNS
SETOF jobs AS $$
                          BEGIN
                              RETURN QUERY INSERT INTO jobs(spec,status,object_meta,type_meta )
                                  VALUES (spec,status,object_meta,type_meta)
                                  RETURNING *;
                              RETURN;
                          END
                      $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_jobs_v1() RETURNS
SETOF jobs AS $$
BEGIN
RETURN QUERY SELECT * FROM jobs;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_jobs_by_node_v1(node text) RETURNS
SETOF jobs AS $$
BEGIN
RETURN QUERY SELECT * FROM jobs WHERE spec ->> 'node_id' = node ;
RETURN;
END $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_job_v1(jid bigint) RETURNS
SETOF jobs AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM jobs WHERE id=jid;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION set_job_status_v1 (jid bigint, job_status JSONB) RETURNS
SETOF jobs AS $$
                                        BEGIN
                                            RETURN QUERY UPDATE jobs SET status=job_status, updated_at=now() WHERE id=jid
                                            RETURNING *;
                                            RETURN;
                                        END
                                     $$ LANGUAGE PLPGSQL VOLATILE;
