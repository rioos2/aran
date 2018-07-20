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
      'RIOOS.INGRESSES.*',
      'Any access allowed for this ingress resource.'
)
,
(
  (
  SELECT
     id
  FROM
     first_insert),
     'RIOOS.ASSEMBLYS.*.STATUS.PUT',
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
         'RIOOS.ASSEMBLYFACTORYS.GET',
         'Read access for assemblyfactory resource.'
   )
,
(
(
   SELECT
      id
   FROM
      first_insert),
      'RIOOS.ASSEMBLYFACTORYS.*.GET',
      'Read access for assemblyfactory resource.'
)
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.ASSEMBLYFACTORYS.PUT',
         'Edit access for assemblyfactory resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.ASSEMBLYFACTORYS.DELETE',
         'Delete access for assemblyfactory resource.'
   )
,
(
  (
   SELECT
      id
   FROM
      first_insert),
      'RIOOS.STACKSFACTORYS.GET',
      'Read access for stacksfactorys resource.'
)
,
(
  (
   SELECT
      id
   FROM
      first_insert),
      'RIOOS.STACKSFACTORYS.PUT',
      'Edit access for stacksfactorys resource.'
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
         'RIOOS.HORIZONTALSCALING.*.METRICS.GET',
         'Read only access for horizontalscaling metric resource.'
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
         'RIOOS.ENDPOINTS.*',
         'Any access allowed for this endpoints resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.JOBS.*',
         'Any access allowed for this jobs resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.SERVICES.*',
         'Any access allowed for this service resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.VOLUMES.*',
         'Any access allowed for this volumes resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.NODES.*',
         'Any access allowed for this nodes resource.'
   )
,

(
(
   SELECT
      id
   FROM
      first_insert),
      'RIOOS.SENSEIS.GET',
      'Access for this sensei resource.'
)
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.STORAGECONNECTORS.*',
         'Any access allowed for this storage connectors resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.STORAGESPOOL.*',
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
         'RIOOS.IMAGEREFERENCES.*',
         'Any access allowed for this image reference resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.IMAGEMARKS.*',
         'Any access allowed for this image mark resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.BUILDS.*',
         'Any access allowed for this build resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.BUILDCONFIGS.GET',
         'Read only access for buildconfig resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.BUILDCONFIGS.PUT',
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
      'RIOOS.PLANS.*.GET',
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
         'RIOOS.ACCOUNTS.*.GET',
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
         'RIOOS.NETWORKS.GET',
         'Read only access for networks resource.'
   )
,
(
  (
   SELECT
      id
   FROM
      first_insert),
      'RIOOS.NETWORKS.*.GET',
      'Read only access for networks resource.'
)
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.NETWORKS.PUT',
         'Edit only access for networks resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.AUDITS.POST',
         'Create access for audits resource.'
   )
,
(
(
      SELECT
         id
      FROM
         first_insert),
         'RIOOS.BUCKETS.*',
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
         'RIOOS.ACCOUNTS.BUCKETS.*',
         'Any access allowed for this bucket resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.ASSEMBLYS.*.PUT',
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
         'RIOOS.ORIGINS.RIOOS_SYSTEM.SETTINGSMAP.CLUSTER_INFO.GET',
         'Read only access for origin based settingsmap resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.ACCOUNTS.*.ASSEMBLYFACTORYS.*',
         'Any access allowed for this assemblyfactory resource.'
   )
,
(
  (
   SELECT
      id
   FROM
      second_insert),
      'RIOOS.ACCOUNTS.*.STACKSFACTORYS.*',
      'Any access allowed for this stacksfactory resource.'
)
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.HORIZONTALSCALING.*',
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
         'RIOOS.SECRETS.*',
         'Any access allowed for this secrets resource.'
   )
,
(
(
     SELECT
        id
     FROM
        second_insert),
        'RIOOS.ORIGINS.RIOOS_SYSTEM.SECRETS.AGENT_SECRET.GET',
        'Access allowed for origin to get secrets resource.'
  )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.ENDPOINTS.*',
         'Any access allowed for this endpoints resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.SERVICES.*',
         'Any access allowed for this service resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.BUILDCONFIGS.*',
         'Any access allowed for this buildconfig resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.LOGS.GET',
         'Read only access for logs resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.AUDITS.GET',
         'Read only access for audits resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.VOLUMES.GET',
         'Read only access for volumes resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.ACCOUNTS.GET',
         'Read only access for accounts resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'RIOOS.ACCOUNTS.PUT',
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
         'RIOOS.NETWORKS.GET',
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
