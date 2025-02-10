create table if not exists sync_connections (
  id text primary key,
  password text not null,
  created_at datetime default current_timestamp
);
