CREATE DATABASE test;

CREATE TABLE test(
    id serial NOT NULL PRIMARY KEY,
    name text NOT NULL
);

insert into test(
    name
) values (
    'This is test row 1'
);

insert into test(
    name
) values (
    'This is test row 2'
);


CREATE TABLE course(
    id serial NOT NULL PRIMARY KEY,
    teacher_id integer NOT NULL,
    name varchar(144) NOT NULL,
    time date NOT NULL default now(),
    lang varchar(20)
);

insert into course(
    teacher_id,
    name,
    lang
) values (
    1,
    'Hello',
    'Chinese'
);

insert into course(
    teacher_id,
    name,
    lang
) values (
    1,
    'World',
    'Chinese'
);

insert into course(
    teacher_id,
    name,
    lang
) values (
    2,
    'Sakura70s',
    'English'
);