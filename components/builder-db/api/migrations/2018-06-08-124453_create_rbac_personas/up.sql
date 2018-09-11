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
      policies(description, type_meta, object_meta, metadata)
   VALUES
      (
         'Role has complete organization control; can manage full organization and overall teams.',
         '{"kind":"Policy","api_version":"v1"}',
         '{"name":"ORG_OWNER"}',
         '{"level":"system"}'
      )
      ON CONFLICT DO NOTHING RETURNING id
)
INSERT INTO
   permissions (policy_id, name, description)
VALUES
   (
(
      SELECT
         id
      FROM
         first_insert),
         'TEAMS.*',
         'Allow every team access to all resources.'
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
      policies(description, type_meta, object_meta, metadata)
   VALUES
      (
         'used  for system level communication via  serviceaccounts (which is Rio/OS system accounts).',
         '{"kind":"Policy","api_version":"v1"}',
         '{"name":"UNIVERSALSOLDIER"}',
         '{"level":"serviceaccount"}'
      )
      ON CONFLICT DO NOTHING RETURNING id
)
INSERT INTO
   permissions (policy_id, name, description)
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
         'LICENSES.GET',
         'Read access for license resource.'
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
      policies(description, type_meta, object_meta, metadata)
   VALUES
      (
         'This is a regular  user used for default permissions for every users ',
         '{"kind":"Policy","api_version":"v1"}',
         '{"name":"LONERANGER"}',
         '{"level":"default"}'
      )
      ON CONFLICT DO NOTHING RETURNING id
)
INSERT INTO
   permissions (policy_id, name, description)
VALUES
(
(
      SELECT
         id
      FROM
         second_insert),
         'LOGS.GET',
         'Read access allowed for this log resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'AUDITS.GET',
         'Read only access for audit resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'SETTINGSMAP.GET',
         'Read only access for settingsmap resource.'
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
      'SESSIONS.GET',
      'Read only access for session resource.'
)
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'TEAMS.GET',
         'Read only access for teams resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'INVITATIONS.PUT',
         'Edit only access for invitation resource.'
   );


   WITH second_insert AS
   (
      INSERT INTO
         policies(description, type_meta, object_meta, metadata)
      VALUES
         (
            ' complete team control, can manage full team and members.',
            '{"kind":"Policy","api_version":"v1"}',
            '{"name":"TEAM_OWNER"}',
            '{"level":"system"}'
         )
         ON CONFLICT DO NOTHING RETURNING id
   )
   INSERT INTO
      permissions (policy_id, name, description)
   VALUES(
(
      SELECT
         id
      FROM
         second_insert),
         'TEAMS.GET',
         'Read only access for teams resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'INVITATIONS.POST',
         'Create access allowed for this invitation resource.'
   )
,
(
  (
   SELECT
      id
   FROM
      second_insert),
      'POLICIES.*',
      'Any access allowed for this policy resource.'
);

WITH second_insert AS
(
   INSERT INTO
      policies(description, type_meta, object_meta, metadata)
   VALUES
      (
         'user used for machine create permissions for every users',
         '{"kind":"Policy","api_version":"v1"}',
         '{"name":"MACHINE_CREATE"}',
         '{"level":"user"}'
      )
      ON CONFLICT DO NOTHING RETURNING id
)
INSERT INTO
   permissions (policy_id, name, description)
VALUES(
(
      SELECT
         id
      FROM
         second_insert),
         'MACHINEFACTORYS.POST',
         'Read access allowed for machine based stacksfactory resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'SECRETS.POST',
         'Create access allowed for this secret resource.'
   )
,
 (
(
      SELECT
         id
      FROM
         second_insert),
         'DATACENTERS.GET',
         'Read only access for datacenters  resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'NETWORKS.GET',
         'Read access allowed for this networks resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'PLANS.GET',
         'Read access allowed for this Plans resource.'
   );


   WITH second_insert AS
   (
      INSERT INTO
         policies(description, type_meta, object_meta, metadata)
      VALUES
         (
            'user used for machine view permissions for every users',
            '{"kind":"Policy","api_version":"v1"}',
            '{"name":"MACHINE_VIEW"}',
            '{"level":"user"}'
         )
         ON CONFLICT DO NOTHING RETURNING id
   )
   INSERT INTO
      permissions (policy_id, name, description)
   VALUES(
(
      SELECT
         id
      FROM
         second_insert),
         'MACHINES.GET',
         'Read access allowed for this machine based assembly resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'SECRETS.GET',
         'Read access allowed for this secret resource.'
   );

   WITH second_insert AS
   (
      INSERT INTO
         policies(description, type_meta, object_meta, metadata)
      VALUES
         (
            'user used for machine delete permissions for every users',
            '{"kind":"Policy","api_version":"v1"}',
            '{"name":"MACHINE_DELETE"}',
            '{"level":"user"}'
         )
         ON CONFLICT DO NOTHING RETURNING id
   )
   INSERT INTO
      permissions (policy_id, name, description)
   VALUES(
(
      SELECT
         id
      FROM
         second_insert),
         'ASSEMBLYS.PUT',
         'Edit only access for machine based assembly resource.'
   );

   WITH second_insert AS
   (
      INSERT INTO
         policies(description, type_meta, object_meta, metadata)
      VALUES
         (
            'user used for conatiner create permissions for every users',
            '{"kind":"Policy","api_version":"v1"}',
            '{"name":"CONTAINER_CREATE"}',
            '{"level":"user"}'
         )
         ON CONFLICT DO NOTHING RETURNING id
   )
   INSERT INTO
      permissions (policy_id, name, description)
   VALUES
   (
(
      SELECT
         id
      FROM
         second_insert),
         'CONTAINERFACTORYS.POST',
         'Create only access for container based stackfactory resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'SECRETS.POST',
         'create only access for secret resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'HORIZONTALSCALING.POST',
         'Create only access for horizontalscaling resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'VERTICALSCALING.POST',
         'Create only access for verticalscaling resource.'
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
            'PLANS.GET',
            'Read only access for plans resource.'
      );

      WITH second_insert AS
      (
         INSERT INTO
            policies(description, type_meta, object_meta, metadata)
         VALUES
            (
               'user used for container view permissions for every users',
               '{"kind":"Policy","api_version":"v1"}',
               '{"name":"CONTAINER_VIEW"}',
               '{"level":"user"}'
            )
            ON CONFLICT DO NOTHING RETURNING id
      )
      INSERT INTO
         permissions (policy_id, name, description)
      VALUES
   (
(
      SELECT
         id
      FROM
         second_insert),
         'CONTAINERS.GET',
         'Read only access for container based assembly resource.'
   )
,
   (
(
      SELECT
         id
      FROM
         second_insert),
         'SECRETS.GET',
         'Read only access for secret resource.'
   );

   WITH second_insert AS
   (
      INSERT INTO
         policies(description, type_meta, object_meta, metadata)
      VALUES
         (
            'user used for container delete permissions for every users',
            '{"kind":"Policy","api_version":"v1"}',
            '{"name":"CONTAINER_DELETE"}',
            '{"level":"user"}'
         )
         ON CONFLICT DO NOTHING RETURNING id
   )
   INSERT INTO
      permissions (policy_id, name, description)
   VALUES(
   (
         SELECT
            id
         FROM
            second_insert),
            'ASSEMBLYS.PUT',
            'Edit access for container based assembly resource.'
      )
;
