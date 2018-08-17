---
--- Table:teams:create stub team named RIOOS:SUPERUSER
--- When editing teams, use uppercase.
---
---
--- Table:permissions:create stub permissions for team RIOOS:SUPERUSER
--- When editing teams, use uppercase.
--- This is a long query.
WITH first_insert AS
(
   INSERT INTO
      teams(full_name, description, type_meta, object_meta, metadata)
   VALUES
      (
         'RIOOS:SUPERUSER',
         'Superuser RIO/OS. God given powers. ',
         '{"kind":"Team","api_version":"v1"}',
         json_build_object()::jsonb,
         json_build_object()::jsonb
      )
      ON CONFLICT (full_name) DO NOTHING RETURNING id
)
INSERT INTO
   permissions (team_id, name, description)
VALUES
   (
(
      SELECT
         id
      FROM
         first_insert),
         '*.*',
         'Allow every access to all resources.'
   )
;


---
--- Table:teams:create stub team named RIOOS:UNIVERSALSOLDIER
--- When editing teams, use uppercase.
---
---
--- Table:permissions:create stub permissions for team RIOOS:UNIVERSALSOLDIER
--- When editing teams, use uppercase.
--- This is a long query.
WITH first_insert AS
(
   INSERT INTO
      teams(full_name, description, type_meta, object_meta, metadata)
   VALUES
      (
         'RIOOS:UNIVERSALSOLDIER',
         'Universalsoldier is system level user (like service account)',
         '{"kind":"Team","api_version":"v1"}',
         json_build_object()::jsonb,
         json_build_object()::jsonb
      )
      ON CONFLICT (full_name) DO NOTHING RETURNING id
)
INSERT INTO
   permissions (team_id, name, description)
VALUES
   (
(
      SELECT
         id
      FROM
         first_insert),
         'ASSEMBLYS.GET',
         'Read access for assembly resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'ASSEMBLYS.PUT',
         'Edit access for assembly resource.'
   )
,
(
  (
   SELECT
      id
   FROM
      first_insert),
      'INGRESSES.*',
      'Any access allowed for this ingress resource.'
)
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'ASSEMBLYFACTORYS.GET',
         'Read access for assemblyfactory resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'ASSEMBLYFACTORYS.PUT',
         'Edit access for assemblyfactory resource.'
   )
,
(
  (
   SELECT
      id
   FROM
      first_insert),
      'STACKSFACTORYS.GET',
      'Read access for stacksfactorys resource.'
)
,
(
  (
   SELECT
      id
   FROM
      first_insert),
      'STACKSFACTORYS.PUT',
      'Edit access for stacksfactorys resource.'
)
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'SERVICEACCOUNTS.PUT',
         'Edit only access for service account resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'SERVICEACCOUNTS.DELETE',
         'Delete access for service account resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'HORIZONTALSCALING.GET',
         'Read only access for horizontalscaling resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'HORIZONTALSCALING.PUT',
         'Edit only access for horizontalscaling resource.'
   )
,
 (
(
      SELECT
         id
      FROM
         first_insert),
         'METRICS.GET',
         'Read only access for horizontalscaling metric resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'VERTICALSCALING.GET',
         'Read only access for verticalscaling resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'VERTICALSCALING.PUT',
         'Edit only access for verticalscaling resource.'
   )
,  
   (
(
      SELECT
         id
      FROM
         first_insert),
         'SECRETS.*',
         'Any access allowed for this secrets resource.'
   )
,
   (
(
      SELECT
        id
      FROM
         first_insert),
         'ENDPOINTS.*',
         'Any access allowed for this endpoints resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'JOBS.*',
         'Any access allowed for this jobs resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'SERVICES.*',
         'Any access allowed for this service resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'VOLUMES.*',
         'Any access allowed for this volumes resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'NODES.*',
         'Any access allowed for this nodes resource.'
   )
,

(
(
   SELECT
      id
   FROM
      first_insert),
      'SENSEIS.GET',
      'Access for this sensei resource.'
)
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'STORAGECONNECTORS.*',
         'Any access allowed for this storage connectors resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'STORAGESPOOL.*',
         'Any access allowed for this storage pool resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'SETTINGSMAP.*',
         'Any access allowed for this settings map resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'IMAGEREFERENCES.*',
         'Any access allowed for this image reference resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'IMAGEMARKS.*',
         'Any access allowed for this image mark resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'BUILDS.*',
         'Any access allowed for this build resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'BUILDCONFIGS.GET',
         'Read only access for buildconfig resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'BUILDCONFIGS.PUT',
         'Edit only access for buildconfig resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'PLANS.GET',
         'Read only access for plans resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'PLANS.PUT',
         'Edit only access for plans resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'ACCOUNTS.GET',
         'Read only access for accounts resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'DATACENTERS.GET',
         'Read only access for datacenters resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'DATACENTERS.PUT',
         'Edit only access for datacenters resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'NETWORKS.GET',
         'Read only access for networks resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'NETWORKS.PUT',
         'Edit only access for networks resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         first_insert),
         'AUDITS.POST',
         'Create access for audits resource.'
   )
,
(
(
      SELECT
         id
      FROM
         first_insert),
         'BUCKETS.*',
         'Any access allowed for this bucket resource.'
   )
;


---
--- Table:teams:create stub team named RIOOS:LONERANGER
--- When editing teams, use uppercase.
---
---
--- Table:permissions:create stub permissions for team RIOOS:LONERANGER
--- When editing teams, use uppercase.
--- This is a long query.
WITH second_insert AS
(
   INSERT INTO
      teams(full_name, description, type_meta, object_meta, metadata)
   VALUES
      (
         'RIOOS:LONERANGER',
         'This is a regular  user ',
         '{"kind":"Team","api_version":"v1"}',
         json_build_object()::jsonb,
         json_build_object()::jsonb
      )
      ON CONFLICT (full_name) DO NOTHING RETURNING id
)
INSERT INTO
   permissions (team_id, name, description)
VALUES
(
(
      SELECT
         id
      FROM
         second_insert),
         'BUCKETS.*',
         'Any access allowed for this bucket resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'ASSEMBLYS.PUT',
         'Edit only access for assembly resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'TEAMS.*',
         'Edit only access for assembly resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'ASSEMBLYS.GET',
         'Read only access for assembly resource.'
   )
,
 (
(
      SELECT
         id
      FROM
         second_insert),
         'SETTINGSMAP.GET',
         'Read only access for origin based settingsmap resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'ASSEMBLYFACTORYS.*',
         'Any access allowed for this assemblyfactory resource.'
   )
,
(
  (
   SELECT
      id
   FROM
      second_insert),
      'STACKSFACTORYS.*',
      'Any access allowed for this stacksfactory resource.'
)
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'HORIZONTALSCALING.*',
         'Any access allowed for this horizontalscaling resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'VERTICALSCALING.*',
         'Any access allowed for this verticalscaling resource.'
   )
,
 (
(
      SELECT
         id
      FROM
         second_insert),
         'METRICS.GET',
         'Read only access for horizontalscaling metric resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'SECRETS.*',
         'Any access allowed for this secrets resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'ENDPOINTS.*',
         'Any access allowed for this endpoints resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'SERVICES.*',
         'Any access allowed for this service resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'BUILDCONFIGS.*',
         'Any access allowed for this buildconfig resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'LOGS.GET',
         'Read only access for logs resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'AUDITS.GET',
         'Read only access for audits resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'VOLUMES.GET',
         'Read only access for volumes resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'ACCOUNTS.GET',
         'Read only access for accounts resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'ACCOUNTS.PUT',
         'Edit only access for accounts resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'DATACENTERS.GET',
         'Read only access for datacenters resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'HEALTHZ.GET',
         'Read only access for healthz resource.'
   )
   ,
      (
   (
         SELECT
            id
         FROM
            second_insert),
            'HEALTHZ.OVERALL.GET',
            'Read only access for healthz overall resource.'
      )
   ,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'PLANS.GET',
         'Read only access for plan resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'NETWORKS.GET',
         'Read only access for networks resource.'
   )
   ,
      (
   (
         SELECT
            id
         FROM
            second_insert),
            'PING.POST',
            'Create access for ping resource.'
      )
;