#!/usr/bin/python
# -*- coding: utf-8 -*-
from locust import HttpLocust, TaskSet, task, events


class AuthBrowsingUser(TaskSet):

    def on_start(l):
        l.register()

    @task(1)
    def register(l):
		l.client.verify = False
		resp = l.client.post('/api/v1/authenticate',
												json={"email":"info@riocorp.io","password":"team4riocorp"},
                        						cert= ("/home/suganya/nilavu_config/config/config/server-ca.cert.pem", "/home/suganya/nilavu_config/config/config/server-ca.key","/home/suganya/nilavu_config/config/config/client-ca.pub","/home/suganya/nilavu_config/config/config/client-ca.key")
                        						)
        	l.tok = resp.json()['token']

    @task(2)
    def page404(l):
        l.client.get('/does_not_exist')

    @task(3)
    def frontpage(l):
        response = l.client.get('/', name='ARAN Version')


class WebsiteAuthUser(HttpLocust):

    task_set = AuthBrowsingUser
