---
--- Table:roles:create stub role named RIOOS:SUPERUSER
--- When editing roles, use uppercase.
---
---
--- Table:permissions:create stub permissions for role RIOOS:SUPERUSER
--- When editing roles, use uppercase.
--- This is a long query.
WITH first_insert AS
(
   INSERT INTO
      roles(name, description)
   VALUES
      (
         'RIOOS:SUPERUSER',
         'Superuser RIO/OS. God given powers. '
      )
      ON CONFLICT (name) DO NOTHING RETURNING id
)
INSERT INTO
   permissions (role_id, name, description)
VALUES
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.*.*',
         'Allow every access to all resources.'
   )
;


---
--- Table:roles:create stub role named RIOOS:UNIVERSALSOLDIER
--- When editing roles, use uppercase.
---
---
--- Table:permissions:create stub permissions for role RIOOS:UNIVERSALSOLDIER
--- When editing roles, use uppercase.
--- This is a long query.
WITH first_insert AS
(
   INSERT INTO
      roles(name, description)
   VALUES
      (
         'RIOOS:UNIVERSALSOLDIER',
         'Universalsoldier is system level user (like service account)'
      )
      ON CONFLICT (name) DO NOTHING RETURNING id
)
INSERT INTO
   permissions (role_id, name, description)
VALUES
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.ASSEMBLYS.GET',
         'Read access for assembly resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.ASSEMBLYS.PUT',
         'Edit access for assembly resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.ASSEMBLYS.DELETE',
         'Delete access for assembly resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.ASSEMBLYFACTORY.GET',
         'Read access for assemblyfactory resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.ASSEMBLYFACTORY.PUT',
         'Edit access for assemblyfactory resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.ASSEMBLYFACTORY.DELETE',
         'Delete access for assemblyfactory resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.SERVICEACCOUNTS.PUT',
         'Edit only access for service account resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.SERVICEACCOUNTS.DELETE',
         'Delete access for service account resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.HORIZONTALSCALING.GET',
         'Read only access for horizontalscaling resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.HORIZONTALSCALING.PUT',
         'Edit only access for horizontalscaling resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.VERTICALSCALING.GET',
         'Read only access for verticalscaling resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.VERTICALSCALING.PUT',
         'Edit only access for verticalscaling resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.SECRETS.*',
         'Any access allowed for this secrets resource.'
   )
,
   (
(
      SELECT
        id
      FROM
         first_insert),
         'RIOOS.ENDPOINT.*',
         'Any access allowed for this endpoints resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.JOB.*',
         'Any access allowed for this jobs resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.SERVICE.*',
         'Any access allowed for this service resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.VOLUME.*',
         'Any access allowed for this volumes resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.NODE.*',
         'Any access allowed for this nodes resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.STORAGECONNECTOR.*',
         'Any access allowed for this storage connectors resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.STORAGEPOOL.*',
         'Any access allowed for this storage pool resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.SETTINGSMAP.*',
         'Any access allowed for this settings map resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.IMAGEREFERENCE.*',
         'Any access allowed for this image reference resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.IMAGEMARK.*',
         'Any access allowed for this image mark resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.BUILD.*',
         'Any access allowed for this build resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.BUILDCONFIG.GET',
         'Read only access for buildconfig resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.BUILDCONFIG.PUT',
         'Edit only access for buildconfig resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.PLANS.GET',
         'Read only access for plans resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.PLANS.PUT',
         'Edit only access for plans resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.ACCOUNT.GET',
         'Read only access for accounts resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.DATACENTERS.GET',
         'Read only access for datacenters resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.DATACENTERS.PUT',
         'Edit only access for datacenters resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.NETWORK.GET',
         'Read only access for networks resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.NETWORK.PUT',
         'Edit only access for networks resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.AUDIT.POST',
         'Create access for audits resource.'
   )
,
(
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.BUCKET.*',
         'Any access allowed for this bucket resource.'
   )
;


---
--- Table:roles:create stub role named RIOOS:LONERANGER
--- When editing roles, use uppercase.
---
---
--- Table:permissions:create stub permissions for role RIOOS:LONERANGER
--- When editing roles, use uppercase.
--- This is a long query.
WITH second_insert AS
(
   INSERT INTO
      roles(name, description)
   VALUES
      (
         'RIOOS:LONERANGER',
         'This is a regular  user '
      )
      ON CONFLICT (name) DO NOTHING RETURNING id
)
INSERT INTO
   permissions (role_id, name, description)
VALUES
(
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.ACCOUNTS.BUCKET.*',
         'Any access allowed for this bucket resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.ASSEMBLYS.PUT',
         'Edit only access for assembly resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.ACCOUNTS.*.ASSEMBLYS.GET',
         'Read only access for assembly resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.ACCOUNTS.*.ASSEMBLYFACTORY.*',
         'Any access allowed for this assemblyfactory resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.HORIZONTALSCALNG.*',
         'Any access allowed for this horizontalscaling resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.VERTICALSCALING.*',
         'Any access allowed for this verticalscaling resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.SECRET.*',
         'Any access allowed for this secrets resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.ENDPOINT.*',
         'Any access allowed for this endpoints resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.SERVICE.*',
         'Any access allowed for this service resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.BUILDCONFIG.*',
         'Any access allowed for this buildconfig resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.LOG.GET',
         'Read only access for logs resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.AUDIT.GET',
         'Read only access for audits resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.VOLUME.GET',
         'Read only access for volumes resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.ACCOUNT.GET',
         'Read only access for accounts resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.ACCOUNT.PUT',
         'Edit only access for accounts resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.DATACENTERS.GET',
         'Read only access for datacenters resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.HEALTHZ.GET',
         'Read only access for healthz resource.'
   )
   ,
      (
   (
         SELECT
            id
         FROM
            second_insert),
            'RIOOS.HEALTHZ.OVERALL.GET',
            'Read only access for healthz overall resource.'
      )
   ,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.PLANS.GET',
         'Read only access for plan resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.NETWORK.GET',
         'Read only access for networks resource.'
   )
   ,
      (
   (
         SELECT
            id
         FROM
            second_insert),
            'RIOOS.PING.POST',
            'Create access for ping resource.'
      )
;
