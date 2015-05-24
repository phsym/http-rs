# TODO list

## Features :
* Make openssl optional through feature
* Manage keep-alive / disconnections / reconnections
* Use of "url" library
* Use of "mime" library
* Manage cookies. Maybe with "cookie" library
* Manage http-auth

## Improvements :
* Abstract HTTP clients class implementations
* Optimize header management when sending requests (quite heavy for now when cloning)
* In Streams open methods : Better manage error by replacing calls to unwrap()

## General :
* Learn how to do testing, and add some
* Remove main.rs and only use examples, tests, and doc tests
* Improve modules layout
* Improve documentation
* Publish API documentation
