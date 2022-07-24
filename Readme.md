# Works Manager

## Directory Structure

```plaintext
works-manager
|---src                         Frontend
|   |---components              React Components
|   |---hooks                   React Hooks
|   |---icons                   SVG Icons
|   |---pages                   Pages
|---src-tauri                   Backend
    |---docs
    |---icons
    |---migrations              Diesel Migrations
    |---src
        |---domains             Business Logics
        |   |---value           Entities
        |---infrastructures     Infrastructures
        |   |---database        Database Schema
        |   |---repository      Repository
        |---services            Domain Service
        |---usecases            UseCase
```

## DDD (Dependency)

Entity
↑
Domain ← Infrastructure
↑
UseCase
↑
Service
