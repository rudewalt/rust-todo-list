## Rust todo list api

Todo list api built with Rocket framework with apikey authorization for create/update/delete request

All tasks stored in MongoDB.

### Env variables

- DB_PATH - required, MongoDB connection string
- API_KEYS - required, comma separated list of api-keys

### TODO
- sort tasks by created_at
- add User entity
- add JWT authorization
- add Dockerfile to initialize MongoDB
- add new Response model for error and success responses