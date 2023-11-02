# webapp 

## Backend
```
Rust + Axum
```

## Frontend
```
Typescript + React
```

## To start the backend, postgresdb, and redis and nginx
```
docker-compose up --build
```

## To start the front end
```
cd frontend
pnpm install
pnpm run dev
```

### To create a user table in postgres use the PgAdmin extension or use psql to connect to the db
```
CREATE TABLE IF NOT EXISTS public.users
(
    id character varying COLLATE pg_catalog."default" NOT NULL,
    email character varying COLLATE pg_catalog."default" NOT NULL,
    username character varying COLLATE pg_catalog."default" NOT NULL,
    password character varying COLLATE pg_catalog."default" NOT NULL,
    steam_id character varying COLLATE pg_catalog."default",
    psn_id character varying COLLATE pg_catalog."default",
    xbox_id character varying COLLATE pg_catalog."default",
    CONSTRAINT users_pkey PRIMARY KEY (id),
    CONSTRAINT unique_username UNIQUE (username)
)
```
