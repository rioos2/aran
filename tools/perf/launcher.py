import os

from AranLocust import AranLocust
from AranLocust import LauncherTaskSet


class Launcher(AranLocust.AranLocust):
    host = "127.0.0.1"
    ca_cert_pem = os.path.join(
        os.environ["RIOOS_HOME"] + os.sep + 'config' + os.sep, 'server-ca.cert.pem')
    ca_key = os.path.join(
        os.environ["RIOOS_HOME"] + os.sep + 'config' + os.sep, 'server-ca.key')
    client_ca_pub = os.path.join(
        os.environ["RIOOS_HOME"] + os.sep + 'config' + os.sep, 'client-ca.pub')
    client_ca_key = os.path.join(
        os.environ["RIOOS_HOME"] + os.sep + 'config' + os.sep, 'client-ca.key')
    min_wait = 100
    max_wait = 1000
    weight = 3
    task_set = LauncherTaskSet
