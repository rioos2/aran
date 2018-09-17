---
---
--- Table:accounts
---

CREATE SEQUENCE IF NOT EXISTS account_id_seq;
CREATE TABLE IF NOT EXISTS accounts (id bigint UNIQUE PRIMARY KEY DEFAULT next_id_v1('account_id_seq'), email text UNIQUE, first_name text, last_name text, phone text, api_key text, password text, approval bool, suspend bool, is_admin bool, registration_ip_address text, trust_level text, company_name text, object_meta JSONB, type_meta JSONB, avatar BYTEA, updated_at timestamptz, created_at timestamptz DEFAULT now());

---
--- Table:accounts:create
---
CREATE
OR REPLACE FUNCTION insert_account_v1 (account_email text, account_first_name text, account_last_name text, account_phone text, account_api_key text, account_password text, account_approval bool, account_suspend bool, account_is_admin bool, account_registration_ip_address text, account_trust_level text, account_company_name text, account_object_meta JSONB, account_type_meta JSONB, account_avatar BYTEA) RETURNS SETOF accounts AS $$
DECLARE inserted_account accounts;
BEGIN
   SELECT
      * INTO inserted_account
   FROM
      accounts;
IF FOUND
THEN
   INSERT INTO
      accounts ( email, first_name, last_name, phone, api_key, password, approval, suspend, is_admin, registration_ip_address, trust_level, company_name, object_meta, type_meta, avatar)
   VALUES
      (
         account_email, account_first_name, account_last_name, account_phone, account_api_key, account_password, account_approval, account_suspend, account_is_admin, account_registration_ip_address, account_trust_level, account_company_name, account_object_meta, account_type_meta, account_avatar
      )
      ON CONFLICT DO NOTHING RETURNING * INTO inserted_account;
      RETURN NEXT inserted_account;
ELSE
   INSERT INTO
      accounts ( email, first_name, last_name, phone, api_key, password, approval, suspend, is_admin, registration_ip_address, trust_level, company_name, object_meta, type_meta, avatar)
   VALUES
      (
         account_email, account_first_name, account_last_name, account_phone, account_api_key, account_password, account_approval, account_suspend, true, account_registration_ip_address, account_trust_level, account_company_name, account_object_meta, account_type_meta, account_avatar
      )
      ON CONFLICT DO NOTHING RETURNING * INTO inserted_account;
      RETURN NEXT inserted_account;
RETURN;
END
IF;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:accounts:show by admin
---
CREATE
OR REPLACE FUNCTION get_accounts_v1_by_is_admin () RETURNS SETOF accounts AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      accounts where is_admin = true;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:accounts:show
---
CREATE
OR REPLACE FUNCTION get_account_by_id_v1 (UID bigint) RETURNS SETOF accounts AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      accounts
   WHERE
      id = uid;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:accounts:list
---
CREATE
OR REPLACE FUNCTION get_account_all_v1 () RETURNS SETOF accounts AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      accounts;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;


---
--- Table:accounts:list
---
CREATE
OR REPLACE FUNCTION update_account_by_id_v1 (aid bigint,account_is_admin bool,account_approval bool,account_suspend bool) RETURNS SETOF accounts AS $$
BEGIN
   RETURN QUERY
   UPDATE
      accounts
   SET
      is_admin = account_is_admin,
      approval = account_approval,
      suspend = account_suspend,
      updated_at = now()
   WHERE
      id = aid  RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:accounts:list_by_email
---
CREATE
OR REPLACE FUNCTION get_account_by_email_v1 (account_email text) RETURNS SETOF accounts AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      accounts
   WHERE
      email = account_email;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:account_sessions
---
CREATE TABLE IF NOT EXISTS account_sessions (account_id bigint REFERENCES accounts(id), token text, device JSONB, provider text, is_admin bool DEFAULT FALSE, is_service_access bool DEFAULT FALSE, created_at timestamptz DEFAULT now(), expires_at timestamptz DEFAULT now() + interval '1 day');

---
--- Table:account_sessions:create
---

CREATE
OR REPLACE FUNCTION insert_account_session_v1 (a_account_id bigint, account_token text, account_provider text, device JSONB) RETURNS SETOF account_sessions AS $$
BEGIN
   RETURN QUERY
   INSERT INTO
      account_sessions (account_id, token, provider, device)
   VALUES
      (
         a_account_id,
         account_token,
         account_provider,
         device
      )
      RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:account_sessions:list_blank
---
CREATE
OR REPLACE FUNCTION get_session_v1 (acc_id bigint, acc_device JSONB) RETURNS SETOF account_sessions AS $$
BEGIN
   RETURN QUERY
   SELECT
      *
   FROM
      account_sessions
   WHERE
      account_id = acc_id
      AND acc_device = device LIMIT 1;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:account_sessions:show
---
CREATE
OR REPLACE FUNCTION get_account_session_by_email_token_v1 (account_email text, account_token text) RETURNS TABLE(id bigint, email text, api_key text, token text) AS $$
DECLARE this_account accounts % rowtype;
BEGIN
   SELECT
      *
   FROM
      accounts
   WHERE
      accounts.email = account_email LIMIT 1 INTO this_account;
IF FOUND
THEN
   DELETE
   FROM
      account_sessions
   WHERE
      account_id = this_account.id
      AND account_sessions.token = account_token
      AND expires_at < now();
RETURN QUERY
SELECT
   accounts.id,
   accounts.email,
   accounts.api_key,
   account_sessions.token
FROM
   accounts
   INNER JOIN
      account_sessions
      ON accounts.id = account_sessions.account_id
WHERE
   accounts.id = this_account.id
   AND account_sessions.token = account_token;
END
IF;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:accounts:logout
--- This procedure references account_sessions, hence called after account_sessions
--- creation.

CREATE
OR REPLACE FUNCTION get_logout_v1 (account_email text, account_token text, acc_device JSONB) RETURNS SETOF accounts AS $$
DECLARE this_account accounts % rowtype;
BEGIN
   SELECT
      *
   FROM
      accounts
   WHERE
      accounts.email = account_email LIMIT 1 INTO this_account;
IF FOUND
THEN
   DELETE
   FROM
      account_sessions
   WHERE
      account_id = this_account.id
      AND account_sessions.token = account_token
      AND account_sessions.device = acc_device;
RETURN QUERY
SELECT
   *
from
   accounts
WHERE
   accounts.id = this_account.id;
END
IF;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;
