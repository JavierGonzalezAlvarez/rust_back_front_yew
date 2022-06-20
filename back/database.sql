create table tbemployee(
    id serial primary key,
    name varchar(50) not null,
    phone varchar(10),
    email varchar(50) not null,
    comments varchar(500) not null
);