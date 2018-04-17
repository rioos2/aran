-- Your SQL goes here

CREATE SEQUENCE IF NOT EXISTS account_id_seq;


CREATE TABLE IF NOT EXISTS accounts (id bigint UNIQUE PRIMARY KEY DEFAULT next_id_v1('account_id_seq'),
                                                                          email text UNIQUE,
                                                                                     first_name text, last_name text, phone text, api_key text, password text, approval bool,
                                                                                                                                                               suspend bool,
                                                                                                                                                               ROLES text[], registration_ip_address text, trust_level text, company_name text, object_meta JSONB,
                                                                                                                                                                                                                                                            type_meta JSONB,
                                                                                                                                                                                                                                                                      avatar BYTEA,
                                                                                                                                                                                                                                                                             updated_at timestamptz,
                                                                                                                                                                                                                                                                             created_at timestamptz DEFAULT now());


CREATE OR REPLACE FUNCTION select_or_insert_account_v1 (account_email text, account_first_name text, account_last_name text, account_phone text, account_api_key text, account_password text, account_approval bool, account_suspend bool, account_roles text[], account_registration_ip_address text, account_trust_level text, account_company_name text, account_object_meta JSONB, account_type_meta JSONB, account_avatar BYTEA) RETURNS
SETOF accounts AS $$
                 DECLARE
                    existing_account accounts%rowtype;
                 BEGIN
                    SELECT * INTO existing_account FROM accounts WHERE email = account_email LIMIT 1;
                    IF FOUND THEN
                        RETURN NEXT existing_account;
                    ELSE
                          RETURN QUERY INSERT INTO accounts ( email, first_name, last_name, phone, api_key, password, approval, suspend, roles,registration_ip_address,trust_level,company_name,object_meta,type_meta,avatar)
                         VALUES (account_email, account_first_name, account_last_name, account_phone, account_api_key, account_password,
                             account_approval, account_suspend, account_roles,account_registration_ip_address,account_trust_level, account_company_name, account_object_meta, account_type_meta,account_avatar) ON CONFLICT DO NOTHING RETURNING *;
                    END IF;
                    RETURN;
                 END
             $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION select_only_account_v1 (account_email text, account_first_name text, account_last_name text, account_phone text, account_api_key text, account_password text, account_approval bool, account_suspend bool, account_roles text[], account_registration_ip_address text, account_trust_level text, account_company_name text, account_object_meta JSONB, account_type_meta JSONB, account_avatar BYTEA) RETURNS
SETOF accounts AS $$
                           BEGIN
                              RETURN QUERY SELECT * FROM accounts WHERE email = account_email;
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
                                                                          token text, provider text, is_admin bool DEFAULT FALSE,
                                                                                                                           is_service_access bool DEFAULT FALSE,
                                                                                                                                                          created_at timestamptz DEFAULT now(),
                                                                                                                                                                                         expires_at timestamptz DEFAULT now() + interval '1 day',
                                                                                                                                                                                                                                         UNIQUE (account_id));


CREATE OR REPLACE FUNCTION insert_account_session_v1 (a_account_id bigint, account_token text, account_provider text, account_is_admin bool, account_is_service_access bool) RETURNS
SETOF account_sessions AS $$
BEGIN
RETURN QUERY INSERT INTO account_sessions (account_id, token, provider, is_admin, is_service_access)
                                                                                                               VALUES (a_account_id, account_token, account_provider, account_is_admin, account_is_service_access)
                                                                                                               ON CONFLICT (account_id) DO UPDATE
                                                                                                               SET token = account_token, expires_at = now() + interval '1 day', provider = account_provider, is_admin = account_is_admin, is_service_access = account_is_service_access
                                                                                                               RETURNING *;
                                                                                               RETURN;
                                                                                            END
                                                                                        $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_account_session_v1 (account_email text, account_token text) RETURNS TABLE(id bigint, email text, token text, api_key text, is_admin bool, is_service_access bool) AS $$
      DECLARE
      this_account accounts%rowtype;
      BEGIN
      SELECT * FROM accounts WHERE accounts.email = account_email LIMIT 1 INTO this_account;
      IF FOUND THEN
      DELETE FROM account_sessions WHERE account_id = this_account.id AND account_sessions.token = account_token AND expires_at < now();
      IF NOT FOUND THEN
      RETURN QUERY
      SELECT accounts.id, accounts.email, accounts.api_key, account_sessions.token,account_sessions.is_admin,
      account_sessions.is_service_access
      FROM accounts
      INNER JOIN account_sessions ON account_sessions.account_id = accounts.id
      WHERE accounts.id = this_account.id
      AND account_sessions.token = account_token;
      END IF;
      END IF;
      RETURN;
      END
      $$ LANGUAGE PLPGSQL VOLATILE;
