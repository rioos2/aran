#!/usr/bin/python
# -*- coding: utf-8 -*-
from locust import HttpLocust, TaskSet, task, events


class AuthBrowsingUser(TaskSet):

    def on_start(l):
        l.register()

    @task(1)
    def register(l):
        self.client.post('/authenticate/', {'username': 'username',
                         'password': 'password'},
                         headers={'X-RIOOS-EMAIL': 'info1@megam.io'})
        self.tok = resp.json()['access_token']['token']

    @task(2)
    def page404(l):
        l.client.get('/does_not_exist')

    @task(3)
    def frontpage(l):
        response = l.client.get('/', name='ARAN Version')


class WebsiteAuthUser(HttpLocust):

    task_set = AuthBrowsingUser



			