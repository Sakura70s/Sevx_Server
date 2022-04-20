------------------------------------------------------------------------------------------------------------------------
--    常用命令
------------------------------------------------------------------------------------------------------------------------

-- -- 进入数据库： sudo -u postgres psql -d sevenbox
-- -- Password: 1314159++
-- -- 改密码 	ALTER USER postgres WITH PASSWORD  '1314159++' ;
-- -- \l : 列出所有数据库
-- -- \c: 连接其他数据库
-- -- \d: 列出当前数据库所有表格
-- -- \du: 列出所有用户

------------------------------------------------------------------------------------------------------------------------
--    创建数据库
------------------------------------------------------------------------------------------------------------------------
-- 使用 \c + sevenbox 进入数据库
CREATE DATABASE SevenBox;

------------------------------------------------------------------------------------------------------------------------
--    创建枚举类型
--    现在好像没用了，数据库枚举类型 Sqlx库不支持
------------------------------------------------------------------------------------------------------------------------
-- 字幕枚举类型
CREATE TYPE subTypeENUM AS ENUM (
    '无',
    '外挂',
    '内挂',
    '内嵌'
);
-- 短视频枚举类型
CREATE TYPE svTypeENUM AS ENUM (
    'MV',
    'AMV',
    'SV'
);

------------------------------------------------------------------------------------------------------------------------
--    创建表
------------------------------------------------------------------------------------------------------------------------
-- Animation表
CREATE TABLE Animation(
    id serial NOT NULL PRIMARY KEY,
    seriesFlag boolean NOT NULL,
    seriesId smallint NOT NULL Default 0,
    Animation_name varchar(50) NOT NULL,
    Animation_year date NOT NULL,
    director varchar(20) NOT NULL,
    screenWriter varchar(20) NOT NULL,
    make varchar(20) NOT NULL,
    logo text NOT NULL,
    amount smallint NOT NULL,
    localFlag boolean NOT NULL,
    localUrl varchar(50),
    remoteFlag boolean NOT NULL,
    remoteUrl text,
    container varchar(10) NOT NULL,
    codev varchar(10) NOT NULL,
    codea varchar(10) NOT NULL,
    subType varchar(10) NOT NULL,
    subTeam varchar(20),
    lastWatch date NOT NULL Default '2012-12-12',
    updateTime date NOT NULL Default now(),
    remark text
);
-- 添加注释
COMMENT ON column Animation.id is 'id';
COMMENT ON column Animation.seriesFlag is '系列Flag';
COMMENT ON column Animation.seriesId is '系列ID';
COMMENT ON column Animation.Animation_name is '名称';
COMMENT ON column Animation.Animation_year is '年份';
COMMENT ON column Animation.director is '监督';
COMMENT ON column Animation.screenWriter is '系列构成';
COMMENT ON column Animation.make is '动画制作';
COMMENT ON column Animation.logo is '剧照';
COMMENT ON column Animation.amount is '集数';
COMMENT ON column Animation.localFlag is '本地Flag';
COMMENT ON column Animation.localUrl is '本地存放位置';
COMMENT ON column Animation.remoteFlag is '远程Flag';
COMMENT ON column Animation.remoteUrl is '远程位置';
COMMENT ON column Animation.container is '容器格式';
COMMENT ON column Animation.codev is '本地视频编码格式';
COMMENT ON column Animation.codea is '本地音频编码格式';
COMMENT ON column Animation.subType is '字幕类型';
COMMENT ON column Animation.subTeam is '字幕组';
COMMENT ON column Animation.lastWatch is '最后观看时间';
COMMENT ON column Animation.updateTime is '更新时间';
COMMENT ON column Animation.remark is '备注';


-- Film 表
CREATE TABLE Film(
    id serial NOT NULL PRIMARY KEY,
    seriesFlag boolean NOT NULL,
    seriesId smallint,
    Film_name varchar(50) NOT NULL,
    Film_year date NOT NULL,
    director varchar(20) NOT NULL,
    screenWriter varchar(20) NOT NULL,
    make varchar(20) NOT NULL,
    logo text NOT NULL,
    localFlag boolean NOT NULL,
    localUrl varchar(50),
    remoteFlag boolean NOT NULL,
    remoteUrl text,
    container varchar(10) NOT NULL,
    codev varchar(10) NOT NULL,
    codea varchar(10) NOT NULL,
    subType varchar(10) NOT NULL,
    subTeam varchar(20),
    lastWatch date,
    updateTime date,
    remark text
);
-- 添加注释
COMMENT ON column Film.id is 'id';
COMMENT ON column Film.seriesFlag is '系列Flag';
COMMENT ON column Film.seriesId is '系列ID';
COMMENT ON column Film.Film_name is '名称';
COMMENT ON column Film.Film_year is '年份';
COMMENT ON column Film.director is '导演';
COMMENT ON column Film.screenWriter is '编剧';
COMMENT ON column Film.make is '出品方';
COMMENT ON column Film.logo is '剧照';
COMMENT ON column Film.localFlag is '本地Flag';
COMMENT ON column Film.localUrl is '本地存放位置';
COMMENT ON column Film.remoteFlag is '远程Flag';
COMMENT ON column Film.remoteUrl is '远程位置';
COMMENT ON column Film.container is '容器格式';
COMMENT ON column Film.codev is '本地视频编码格式';
COMMENT ON column Film.codea is '本地音频编码格式';
COMMENT ON column Film.subType is '字幕类型';
COMMENT ON column Film.subTeam is '字幕组';
COMMENT ON column Film.lastWatch is '最后观看时间';
COMMENT ON column Film.updateTime is '更新时间';
COMMENT ON column Film.remark is '备注';


-- TV 表
CREATE TABLE TV(
    id serial NOT NULL PRIMARY KEY,
    seriesFlag boolean NOT NULL,
    seriesId smallint,
    TV_name varchar(50) NOT NULL,
    TV_year date NOT NULL,
    director varchar(20) NOT NULL,
    screenWriter varchar(20) NOT NULL,
    make varchar(20) NOT NULL,
    logo text NOT NULL,
    amount smallint NOT NULL,
    localFlag boolean NOT NULL,
    localUrl varchar(50),
    remoteFlag boolean NOT NULL,
    remoteUrl text,
    container varchar(10) NOT NULL,
    codev varchar(10) NOT NULL,
    codea varchar(10) NOT NULL,
    subType varchar(10) NOT NULL,
    subTeam varchar(20),
    lastWatch date,
    updateTime date,
    remark text
);
-- 添加注释
COMMENT ON column TV.id is 'id';
COMMENT ON column TV.seriesFlag is '系列Flag';
COMMENT ON column TV.seriesId is '系列ID';
COMMENT ON column TV.TV_name is '名称';
COMMENT ON column TV.TV_year is '年份';
COMMENT ON column TV.director is '导演';
COMMENT ON column TV.screenWriter is '编剧';
COMMENT ON column TV.make is '出品方';
COMMENT ON column TV.logo is '剧照';
COMMENT ON column TV.amount is '集数';
COMMENT ON column TV.localFlag is '本地Flag';
COMMENT ON column TV.localUrl is '本地存放位置';
COMMENT ON column TV.remoteFlag is '远程Flag';
COMMENT ON column TV.remoteUrl is '远程位置';
COMMENT ON column TV.container is '容器格式';
COMMENT ON column TV.codev is '本地视频编码格式';
COMMENT ON column TV.codea is '本地音频编码格式';
COMMENT ON column TV.subType is '字幕类型';
COMMENT ON column TV.subTeam is '字幕组';
COMMENT ON column TV.lastWatch is '最后观看时间';
COMMENT ON column TV.updateTime is '更新时间';
COMMENT ON column TV.remark is '备注';


-- SV表
CREATE TABLE SV(
    id serial NOT NULL PRIMARY KEY,
    SV_name varchar(50) NOT NULL,
    SV_year date NOT NULL,
    author varchar(20) NOT NULL,
    SV_type varchar(10) NOT NULL,
    localFlag boolean NOT NULL,
    localUrl varchar(50),
    remoteFlag boolean NOT NULL,
    remoteUrl text,
    container varchar(10) NOT NULL,
    codev varchar(10) NOT NULL,
    codea varchar(10) NOT NULL,
    remark text
);
-- 添加注释
COMMENT ON column SV.id is 'ID';
COMMENT ON column SV.SV_name is '名称';
COMMENT ON column SV.SV_year is '年份';
COMMENT ON column SV.author is '作者';
COMMENT ON column SV.SV_type is '类型';
COMMENT ON column SV.localFlag is '本地Flag';
COMMENT ON column SV.localUrl is '本地位置';
COMMENT ON column SV.remoteFlag is '远程Flag';
COMMENT ON column SV.remoteUrl is '远程位置';
COMMENT ON column SV.container is '容器格式';
COMMENT ON column SV.codev is '本地视频编码格式';
COMMENT ON column SV.codea is '本地音频编码格式';
COMMENT ON column SV.remark is '备注';


-- Music表
CREATE TABLE Music(
    id serial NOT NULL PRIMARY KEY,
    Music_name varchar(50) NOT NULL,
    logo text NOT NULL,
    aritst varchar(20) NOT NULL,
    album varchar(20) NOT NULL,
    Music_year date NOT NULL,
    lyrics varchar(20) NOT NULL,
    written varchar(20) NOT NULL,
    localFlag boolean NOT NULL,
    localUrl varchar(50),
    remoteFlag boolean NOT NULL,
    remoteUrl text,
    container varchar(10) NOT NULL,
    lyricType varchar(10) NOT NULL,
    remark text
);
-- 添加注释
COMMENT ON column Music.id is 'ID';
COMMENT ON column Music.Music_name is '名称';
COMMENT ON column Music.logo is '专辑封面';
COMMENT ON column Music.aritst is '艺术家';
COMMENT ON column Music.album is '专辑';
COMMENT ON column Music.Music_year is '年份';
COMMENT ON column Music.lyrics is '作词';
COMMENT ON column Music.written is '作曲';
COMMENT ON column Music.localFlag is '本地Flag';
COMMENT ON column Music.localUrl is '本地位置';
COMMENT ON column Music.remoteFlag is '远程Flag';
COMMENT ON column Music.remoteUrl is '远程位置';
COMMENT ON column Music.container is '容器格式';
COMMENT ON column Music.lyricType is '歌词类型';
COMMENT ON column Music.remark is '备注';


-- Novel 表
CREATE TABLE Novel(
    id serial NOT NULL PRIMARY KEY,
    seriesFlag boolean NOT NULL,
    seriesId smallint,
    Novel_name varchar(50) NOT NULL,
    Novel_year date NOT NULL,
    logo text NOT NULL,
    author varchar(20) NOT NULL,
    Novel_status varchar(20) NOT NULL,
    localFlag boolean NOT NULL,
    localUrl varchar(50),
    remoteFlag boolean NOT NULL,
    remoteUrl text,
    container varchar(10) NOT NULL,
    remark text
);
-- 添加注释
COMMENT ON column Novel.id is 'ID';
COMMENT ON column Novel.seriesFlag is '系列Flag';
COMMENT ON column Novel.seriesId is '系列ID';
COMMENT ON column Novel.Novel_name is '名称';
COMMENT ON column Novel.Novel_year is '年份';
COMMENT ON column Novel.logo is '封面';
COMMENT ON column Novel.author is '作者';
COMMENT ON column Novel.Novel_status is '连载状态';
COMMENT ON column Novel.localFlag is '本地Flag';
COMMENT ON column Novel.localUrl is '本地位置';
COMMENT ON column Novel.remoteFlag is '远程Flag';
COMMENT ON column Novel.remoteUrl is '远程位置';
COMMENT ON column Novel.container is '容器格式';
COMMENT ON column Novel.remark is '备注';


-- Comic 表
CREATE TABLE Comic(
    id serial NOT NULL PRIMARY KEY,
    seriesFlag boolean NOT NULL,
    seriesId smallint,
    Comic_name varchar(50) NOT NULL,
    Comic_year date NOT NULL,
    logo text NOT NULL,
    author varchar(20) NOT NULL,
    Comic_status varchar(20) NOT NULL,
    localFlag boolean NOT NULL,
    localUrl varchar(50),
    remoteFlag boolean NOT NULL,
    remoteUrl text,
    container varchar(10) NOT NULL,
    remark text
);
-- 添加注释
COMMENT ON column Comic.id is 'ID';
COMMENT ON column Comic.seriesFlag is '系列Flag';
COMMENT ON column Comic.seriesId is '系列ID';
COMMENT ON column Comic.Comic_name is '名称';
COMMENT ON column Comic.Comic_year is '年份';
COMMENT ON column Comic.logo is '封面';
COMMENT ON column Comic.author is '作者';
COMMENT ON column Comic.Comic_status is '连载状态';
COMMENT ON column Comic.localFlag is '本地Flag';
COMMENT ON column Comic.localUrl is '本地位置';
COMMENT ON column Comic.remoteFlag is '远程Flag';
COMMENT ON column Comic.remoteUrl is '远程位置';
COMMENT ON column Comic.container is '容器格式';
COMMENT ON column Comic.remark is '备注';


------------------------------------------------------------------------------------------------------------------------
--    插入测试数据
------------------------------------------------------------------------------------------------------------------------
-- Animation
Insert into Animation (
    seriesFlag,
    seriesId,
    Animation_name,
    Animation_year,
    director,
    screenWriter,
    make,
    logo,
    amount,
    localFlag,
    localUrl,
    remoteFlag,
    remoteUrl,
    container,
    codev,
    codea,
    subType,
    subTeam,
    remark
) Values (
    True,
    1,
    'CLANNAD',
    '2007-10-04',
    '石原立也',
    '志茂文彦',
    'Kyoto Animation',
    'https://static.7os.top/Image/Sakura.png',
    '24',
    True,
    'Disk::M::/BD/CLANNAD',
    False,
    '',
    'mkv',
    'H264',
    'Flac',
    '内挂',
    '澄空学园',
    ''
);


Insert into Animation (
    seriesFlag,
    seriesId,
    Animation_name,
    Animation_year,
    director,
    screenWriter,
    make,
    logo,
    amount,
    localFlag,
    localUrl,
    remoteFlag,
    remoteUrl,
    container,
    codev,
    codea,
    subType,
    subTeam,
    remark
) Values (
    True,
    1,
    'CLANNAD~After Story~',
    '2008-10-02',
    '石原立也',
    '志茂文彦',
    'Kyoto Animation',
    'https://static.7os.top/Image/Sakura.png',
    '25',
    True,
    'Disk::M::/BD/CLANNAD2',
    False,
    '',
    'mkv',
    'H264',
    'Flac',
    '内挂',
    '澄空学园',
    '这是一个测试备注'
);
-- Film

-- TV

-- SV

-- Music

-- Novel

-- Comic