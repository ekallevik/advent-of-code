create table if not exists event
(
    year    int  not null primary key,
    mission text not null
);

create table if not exists solution
(
    date         text not null,
    part         text not null,
    result       text not null,
    is_correct   int  not null,
    duration     int not null,
    processed_at timestamp not null
);
