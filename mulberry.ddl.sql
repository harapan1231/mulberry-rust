create table t_anken (
  anken_id serial not null,
  name varchar(60) not null,
  customer_id char(8) not null,
  user_id char(6) not null,
  status char(1) not null default '0',
  note varchar(250),
  created_at timestamp default current_timestamp,
  created_by char(6),
  updated_at timestamp,
  updated_by char(6),
  primary key(anken_id)
);

create table t_anken_ver (
  anken_id integer not null,
  anken_ver serial not null,
  name varchar(40) not null,
  user_id char(6) not null,
  term_from date default current_date,
  term_to date default current_date,
  status char(1) not null default '0',
  note varchar(250),
  created_at timestamp default current_timestamp,
  created_by char(6),
  updated_at timestamp,
  updated_by char(6),
  primary key(anken_id, anken_ver)
);

create table t_anken_ver_phase (
  anken_id integer not null,
  anken_ver integer not null,
  phase_id integer not null,
  name varchar(12) not null,
  is_main boolean not null default false,
  scale_pct_per_main integer not null default 100,
  unit_price integer not null default 0,
  person_days double precision not null default 0.0,
  term_from date not null default current_date,
  term_to date not null default current_date,
  members integer not null default 0,
  created_at timestamp default current_timestamp,
  created_by char(6),
  updated_at timestamp,
  updated_by char(6),
  primary key(anken_id, anken_ver, phase_id)
);

create table t_anken_ver_func_group (
  anken_id integer not null,
  anken_ver integer not null,
  func_group char(2) not null,
  name varchar(12) not null,
  created_at timestamp default current_timestamp,
  created_by char(6),
  updated_at timestamp,
  updated_by char(6),
  primary key(anken_id, anken_ver, func_group)
);

create table t_anken_ver_func_weight (
  anken_id integer not null,
  anken_ver integer not null,
  func_type char(2) not null,
  weight_type char(1) not null,
  person_days double precision not null default 0.0,
  created_at timestamp default current_timestamp,
  created_by char(6),
  updated_at timestamp,
  updated_by char(6),
  primary key(anken_id, anken_ver, func_type, weight_type)
);

create table t_anken_ver_phase_func (
  anken_id integer not null,
  anken_ver integer not null,
  phase_id integer not null,
  func_id serial not null,
  name varchar(30) not null,
  func_group char(2) not null,
  func_type char(2) not null,
  weight_type char(1) not null,
  note varchar(250),
  created_at timestamp default current_timestamp,
  created_by char(6),
  updated_at timestamp,
  updated_by char(6),
  primary key(anken_id, anken_ver, phase_id, func_id)
);

create table m_customer (
  customer_id char(8) not null,
  name varchar(100) not null,
  created_at timestamp default current_timestamp,
  created_by char(6),
  updated_at timestamp,
  updated_by char(6),
  primary key(customer_id)
);

create table m_user (
  user_id char(6) not null,
  name varchar(30) not null,
  created_at timestamp default current_timestamp,
  created_by char(6),
  updated_at timestamp,
  updated_by char(6),
  primary key(user_id)
);

--status, func_type, weight
create table m_type (
  type_group varchar(8) not null,
  type_id char(1) not null,
  description varchar(12) not null,
  created_at timestamp default current_timestamp,
  created_by char(6),
  updated_at timestamp,
  updated_by char(6),
  primary key(type_group, type_id)
);

