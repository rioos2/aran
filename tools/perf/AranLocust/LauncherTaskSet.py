#!/usr/bin/python
# -*- coding: utf-8 -*-
from locust import HttpLocust, TaskSet, task, events

# The Laucher task set that deals with the performs of
# 1. AssemblyFactory list, update and list again
# 2. Assemblys list, update and list again


class LauncherTaskSet(TaskSet):

    def strapi(self, suffix):
        suffix

    @task(10)
    def assemblyfactorys(self):
        self.client.verify = False
        self.client.get(self.strapi('/api/v1/assemblyfactorys'),
                        headers=self.authenticated_headers)

    @task(5)
    def assemblyfactorys_status_upd(self):
        self.client.verify = False
        self.client.post(
            self.strapi('/api/v1/assemblyfactorys/status'), headers=self.authenticated_headers)

    @task(20)
    def assemblyfactorys_post_status_upd(self):
        self.client.verify = False
        self.client.get(self.strapi('/api/v1/assemblyfactorys'),
                        headers=self.authenticated_headers)

    @task(4)
    def assemblys(self):
        self.client.verify = False
        self.client.get(self.strapi('/api/v1/assemblys'),
                        headers=self.authenticated_headers)

    @task(5)
    def assemblys_status_upd(self):
        self.client.verify = False
        self.client.post(
            self.strapi('/api/v1/assemblys/status'), headers=self.authenticated_headers)

    @task(6)
    def assemblys_post_status_upd(self):
        self.client.verify = False
        self.client.get(
            self.strapi('/api/v1/assemblys'), headers=self.authenticated_headers)

    @task(7)
    def page404(self):
        self.client.get('/does_not_exist')
