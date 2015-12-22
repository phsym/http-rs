# TODO list

## Features :
- [ ] Manage keep-alive / disconnections / reconnections
- [ ] Use of "url" library
- [ ] Use of "mime" library
- [ ] Manage cookies. Maybe with "cookie" library
- [ ] Manage http-auth

## Improvements :
- [ ] Optimize header management when sending requests (quite heavy for now to clone a hashmap)
- [ ] Use a struct for wrapping requests
- [ ] Optimize BufReader and BufWriter usage in Http client implementation

## General :
- [ ] Add unit tests
- [ ] Remove main.rs and only use examples, tests, and doc tests
- [ ] Improve modules layout
- [ ] Improve documentation

## Travis-CI :
- [ ] Automatically publish API documentation with travis after each successfull build
- [ ] Build / Test with and without SSL support
