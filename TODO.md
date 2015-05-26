# TODO list

## Features :
* Make openssl optional through feature
* Manage keep-alive / disconnections / reconnections
* Use of "url" library
* Use of "mime" library
* Manage cookies. Maybe with "cookie" library
* Manage http-auth

## Improvements :
* Optimize header management when sending requests (quite heavy for now to clone a hashmap)
* Use a struct for wrapping requests

## General :
* Learn how to do unit testing in Rust, and add some
* Remove main.rs and only use examples, tests, and doc tests
* Improve modules layout
* Improve documentation
* Publish API documentation
