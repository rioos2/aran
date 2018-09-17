-- Your SQL goes here

CREATE SEQUENCE IF NOT EXISTS account_id_seq;


CREATE TABLE IF NOT EXISTS accounts (id bigint UNIQUE PRIMARY KEY DEFAULT next_id_v1('account_id_seq'),
                                                                          email text UNIQUE,
                                                                                     first_name text, last_name text, phone text, api_key text, password text, approval bool,
                                                                                                                                                               suspend bool,
                                                                                                                                                               is_admin bool, registration_ip_address text, trust_level text, company_name text, object_meta JSONB,
                                                                                                                                                                                                                                                            type_meta JSONB,
                                                                                                                                                                                                                                                                      avatar BYTEA,
                                                                                                                                                                                                                                                                             updated_at timestamptz,
                                                                                                                                                                                                                                                                             created_at timestamptz DEFAULT now());


CREATE OR REPLACE FUNCTION insert_account_v1 (account_email text, account_first_name text, account_last_name text, account_phone text, account_api_key text, account_password text, account_approval bool, account_suspend bool, account_is_admin bool, account_registration_ip_address text, account_trust_level text, account_company_name text, account_object_meta JSONB, account_type_meta JSONB, account_avatar BYTEA) RETURNS
SETOF accounts AS $$
                 BEGIN
                        RETURN QUERY INSERT INTO accounts ( email, first_name, last_name, phone, api_key, password, approval, suspend, is_admin,registration_ip_address,trust_level,company_name,object_meta,type_meta,avatar)
                         VALUES (account_email, account_first_name, account_last_name, account_phone, account_api_key, account_password,
                             account_approval, account_suspend, account_is_admin,account_registration_ip_address,account_trust_level, account_company_name, account_object_meta, account_type_meta,account_avatar) ON CONFLICT DO NOTHING RETURNING *;
                    RETURN;
                 END
             $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_accounts_v1 () RETURNS
SETOF accounts AS $$
                                     BEGIN
                                       RETURN QUERY SELECT * FROM accounts;
                                       RETURN;
                                     END
                                     $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_account_by_id_v1 (UID bigint) RETURNS
SETOF accounts AS $$
                                                   BEGIN
                                                     RETURN QUERY SELECT * FROM accounts WHERE id = uid;
                                                     RETURN;
                                                   END
                                                   $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_account_by_email_v1 (account_email text) RETURNS
SETOF accounts AS $$
                                                                 BEGIN
                                                                    RETURN QUERY SELECT * FROM accounts WHERE email = account_email;
                                                                    RETURN;
                                                                 END
                                                             $$ LANGUAGE PLPGSQL STABLE;


CREATE TABLE IF NOT EXISTS account_sessions (account_id bigint REFERENCES accounts(id),
                                                                          token text, device JSONB,
                                                                                             provider text, is_admin bool DEFAULT FALSE,
                                                                                                                                  is_service_access bool DEFAULT FALSE,
                                                                                                                                                                 created_at timestamptz DEFAULT now(),
                                                                                                                                                                                                expires_at timestamptz DEFAULT now() + interval '1 day');


CREATE OR REPLACE FUNCTION insert_account_session_v1 (a_account_id bigint, account_token text, account_provider text, device JSONB) RETURNS
SETOF account_sessions AS $$
BEGIN
RETURN QUERY INSERT INTO account_sessions (account_id, token, provider, device)
                                                                                                               VALUES (a_account_id, account_token, account_provider, device)
                                                                                                               RETURNING *;
                                                                                               RETURN;
                                                                                            END
                                                                                        $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_session_v1 (acc_id bigint, acc_device JSONB) RETURNS
SETOF account_sessions AS $$
      BEGIN
      RETURN QUERY SELECT * FROM account_sessions WHERE account_id = acc_id AND acc_device = device LIMIT 1;
      RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_account_session_by_email_token_v1 (account_email text, account_token text) RETURNS TABLE(id bigint, email text, api_key text,token text) AS $$
      DECLARE
      this_account accounts%rowtype;
      BEGIN
      SELECT * FROM accounts WHERE accounts.email = account_email LIMIT 1 INTO this_account;
      IF FOUND THEN
      DELETE FROM account_sessions WHERE account_id = this_account.id AND account_sessions.token = account_token AND expires_at < now();
      RETURN QUERY
      SELECT accounts.id, accounts.email, accounts.api_key, account_sessions.token
      FROM accounts
      INNER JOIN account_sessions ON  accounts.id =account_sessions.account_id
      WHERE accounts.id = this_account.id
      AND account_sessions.token = account_token;
      END IF;
      RETURN;
      END
      $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_logout_v1 (account_email text, account_token text,acc_device JSONB) RETURNS
SETOF accounts AS $$
      DECLARE
        this_account accounts%rowtype;
      BEGIN
          SELECT * FROM accounts WHERE accounts.email = account_email LIMIT 1 INTO this_account;
            IF FOUND THEN
              DELETE FROM account_sessions WHERE account_id = this_account.id AND account_sessions.token = account_token AND account_sessions.device = acc_device;
              RETURN QUERY
              SELECT * from accounts WHERE accounts.id = this_account.id;
            END IF;
              RETURN;
            END
            $$ LANGUAGE PLPGSQL VOLATILE;


CREATE SEQUENCE IF NOT EXISTS origin_id_seq;


CREATE TABLE IF NOT EXISTS origins (id bigint UNIQUE PRIMARY KEY DEFAULT next_id_v1('origin_id_seq'),
                                                                         name text UNIQUE,
                                                                                   type_meta JSONB,
                                                                                             object_meta JSONB,
                                                                                                         created_at timestamptz DEFAULT now(),
                                                                                                                                        updated_at timestamptz);


CREATE SEQUENCE IF NOT EXISTS origin_mem_id_seq;


CREATE TABLE IF NOT EXISTS origin_members (id bigint PRIMARY KEY DEFAULT next_id_v1('origin_mem_id_seq'),
                                                                         type_meta JSONB,
                                                                                   object_meta JSONB,
                                                                                               meta_data JSONB,
                                                                                                         created_at timestamptz DEFAULT now(),
                                                                                                                                        updated_at timestamptz);



INSERT INTO origins (name, object_meta, type_meta)
VALUES ('rioos_system',
        '{"name":"rioos_system", "labels": {}, "account": "", "created_at": "", "deleted_at": "", "finalizers": [], "annotations": {}, "cluster_name": "", "initializers": {"result": {"code": 0, "reason": "", "status": "", "details": {"uid": "", "kind": "", "name": "", "group": "", "causes": [], "retry_after_seconds": 0}, "message": "", "type_meta": {"kind": "", "api_version": ""}}, "pending": []}, "owner_references": [{"uid": "", "kind": "", "name": "", "api_version": "", "block_owner_deletion": false}], "deletion_grace_period_seconds": 0}',
        '{"kind":"Origin","api_version":"v1"}') ON CONFLICT (name) DO NOTHING;

CREATE OR REPLACE FUNCTION internal_insert_origin_member_v1 (om_type_meta JSONB, om_obj_meta JSONB, om_meta_data JSONB) RETURNS void AS $$
        BEGIN
           INSERT INTO
              origin_members ( type_meta, object_meta, meta_data)
           VALUES
              (
                 om_type_meta,
                 om_obj_meta,
                 om_meta_data
              )
        ;
        END
        $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION insert_origin_v1 (origin_name text, origin_type_meta JSONB, origin_object_meta JSONB) RETURNS
SETOF origins AS $$
                       DECLARE
                         existing_origin origins%rowtype;
                         inserted_origin origins;
                       BEGIN
                       SELECT * INTO existing_origin FROM origins WHERE name = origin_name LIMIT 1;
                       IF FOUND THEN
                           RETURN NEXT existing_origin;
                       ELSE
                           INSERT INTO origins (name,type_meta,object_meta)
                                  VALUES (origin_name,origin_type_meta,origin_object_meta) ON CONFLICT (name) DO NOTHING RETURNING * into inserted_origin;
                           PERFORM internal_insert_origin_member_v1('{"kind":"OriginMember","api_version":"v1"}',origin_object_meta, json_build_object('origin',inserted_origin.name)::jsonb);
                           RETURN NEXT inserted_origin;
                           RETURN;
                  END IF;
                  RETURN;
               END
                   $$ LANGUAGE PLPGSQL VOLATILE;

CREATE OR REPLACE FUNCTION get_origins_v1() RETURNS
SETOF origins AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM origins;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_origin_v1 (org_name text) RETURNS
SETOF origins AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM origins WHERE name = org_name;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;
