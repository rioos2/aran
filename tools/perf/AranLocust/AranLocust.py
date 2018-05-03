# -*- coding: utf-8 -*-
##############################################################################
import time
import sys
import random
import string

from locust import HttpLocust, events, task, TaskSet

# Generates a random email of the form
# sachin_<8characters>@rioos.sh
#


def email_generator(size=8, chars=string.ascii_uppercase + string.digits):
    return 'sachin_'+''.join(random.choice(chars)
                             for _ in range(size)).lower() + "@rioos.sh"


class AranLocust(HttpLocust):
    port = 7443
    login = email_generator()
    password = "speed123"
    protocol = "https"
    user_id = -1

    def __init__(self, *args, **kwargs):
        super(AranLocust, self).__init__(*args, **kwargs)
        self._connect()

    def _connect(self):
        user_id = None
        self.client.verify = False
        if self.user_id and self.user_id > 0:
            user_id = self.user_id
        ## Do a signup and then proceed to login in.    
        resp = self.client.post('/api/v1/authenticate',
                                json={"email": self.login,
                                      "password": self.password},
                                cert=(self.ca_cert_pem, self.ca_key,
                                      self.client_ca_pub, self.client_ca_key)
                                )
        print("----------\n")
        print(resp)        
        print("-----------\n")
        self.authenticated_headers = {'Authorization': 'Bearer ' + resp.json()['token'],
                                      'X-AUTH-RIOOS-EMAIL': self.login}
