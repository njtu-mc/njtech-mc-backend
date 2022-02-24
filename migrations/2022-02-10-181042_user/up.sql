-- Your SQL goes here
create table users
(
    id             int auto_increment primary key,
    created_at     DATETIME     NOT NULL default CURRENT_TIMESTAMP comment '创建时间',
    updated_at     DATETIME     NOT NULL default CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP comment '最后修改时间',
    mc_id          varchar(100) NOT NULL default '' comment 'minecraft 中的 id',
    mc_name        varchar(50)  NOT NULL default '' comment 'minecraft 中的名字，每次登录时会更新',
    `gender`       int          NOT NULL default 0 comment '性别 1 - 男， 2 - 女， 其他 - 未知',
    `name`         varchar(30) comment '姓名 如果存在表示用户已激活',
    email          varchar(30) comment '邮箱',
    njtech_open_id varchar(30) comment '学号，如果学校不是南京工业大学，则为推荐人学号',
    school         varchar(100) comment '学校',

    KEY `created_at` (created_at),
    KEY `updated_at` (updated_at),
    unique `mc_id` (mc_id),
    unique email (email)
);

