# -*- coding: utf-8 -*-
##############################################################################

from distutils.core import setup

setup(name='AranLocust',
      version='1.0.0',
      description='Easily load test Rio OS using Locust.',
      author='Kishorekumar Neelamegam',
      author_email='kishore.neelamegam@rio.company',
      url='',
      packages=["AranLocust"],
      install_requires=[          
          'locustio',
      ],
      long_description="See the home page for any information: https://gitlab.com/rioos/aran.",
      keywords="aran locust openerplib",
      license="BSD",
      classifiers=[
          "License :: OSI Approved :: BSD License",
          "Programming Language :: Python",
          ],
     )
