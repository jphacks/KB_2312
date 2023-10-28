create table pages (
  id serial not null primary key
  , title varchar(512) not null unique
  , source text not null
  , create_time timestamp not null
  , update_time timestamp not null
);

create table page_revisions (
  id serial not null primary key
  , page_id int not null
  , source text not null
  , author varchar(256) not null
  , create_time timestamp not null
);
