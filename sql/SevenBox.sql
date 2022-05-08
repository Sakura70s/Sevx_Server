------------------------------------------------------------------------------------------------------------------------
--    常用命令
------------------------------------------------------------------------------------------------------------------------

-- -- 进入数据库： sudo -u postgres psql -d sevx
-- -- Password: 1314159++
-- -- 改密码 	ALTER USER postgres WITH PASSWORD  '1314159++' ;
-- -- \l : 列出所有数据库
-- -- \c: 连接其他数据库
-- -- \d: 列出当前数据库所有表格
-- -- \du: 列出所有用户

------------------------------------------------------------------------------------------------------------------------
--    创建数据库
------------------------------------------------------------------------------------------------------------------------
-- 使用 \c + sevx 进入数据库
CREATE DATABASE Sevx;


------------------------------------------------------------------------------------------------------------------------
--    创建表
------------------------------------------------------------------------------------------------------------------------
------------------------------------------------------
--    Auth 表
------------------------------------------------------
CREATE TABLE Auth(
    id serial NOT NULL PRIMARY KEY,
    uname varchar(20) NOT NULL,
    upassword varchar(64) NOT NULL
);
Insert into Auth(
    uname,
    upassword
) Values (
    'Sakura70s',
    '384fde3636e6e01e0194d2976d8f26410af3e846e573379cb1a09e2f0752d8cc'
);
Insert into Auth(
    uname,
    upassword
) Values (
    'test',
    '8d969eef6ecad3c29a3a629280e686cf0c3f5d5a86aff3ca12020c923adc6c92'
);
------------------------------------------------------
--    Animation 表
------------------------------------------------------
CREATE TABLE Animation(
--  id
    id serial NOT NULL PRIMARY KEY,
--  系列Flag
    seriesFlag boolean NOT NULL,
--  系列ID
    seriesId smallint NOT NULL Default 0,
--  动画名称
    Animation_name varchar(50) NOT NULL,
--  动画年份
    Animation_year date NOT NULL,
--  监督
    director varchar(20) NOT NULL,
--  系列构成
    screenWriter varchar(20) NOT NULL,
--  动画制作
    make varchar(20) NOT NULL,
--  封面
    logo text NOT NULL,
--  集数
    amount smallint NOT NULL,
--  本地标志
    localFlag boolean NOT NULL,
--  本地URL
    localUrl varchar(50),
--  远程标志
    remoteFlag boolean NOT NULL,
--  远程URL
    remoteUrl text,
--  容器格式
    container varchar(10) NOT NULL,
--  视频编码格式
    codev varchar(10) NOT NULL,
--  音频编码格式
    codea varchar(10) NOT NULL,
--  字幕类型
    subType varchar(10) NOT NULL,
--  字幕组
    subTeam varchar(20),
--  最后观看时间
    lastWatch date NOT NULL Default '2012-12-12',
--  更新时间
    updateTime date NOT NULL Default now(),
--  备注
    remark text
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
    'CLANNAD',
    '2007-10-04',
    '石原立也',
    '志茂文彦',
    'Kyoto Animation',
    'https://static.7os.top/Image/Sevx/APPTest/黎歌Neeko_91500837.jpg',
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
    'https://static.7os.top/Image/Sevx/APPTest/黎歌Neeko_91500837.jpg',
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


------------------------------------------------------
--    Fiml 表
------------------------------------------------------
CREATE TABLE Film(
--  id
    id serial NOT NULL PRIMARY KEY,
--  系列Flag
    seriesFlag boolean NOT NULL,
--  系列ID
    seriesId smallint NOT NULL Default 0,
--  电影名称
    Film_name varchar(50) NOT NULL,
--  电影年份
    Film_year date NOT NULL,
--  导演
    director varchar(50) NOT NULL,
--  编剧
    screenWriter varchar(50) NOT NULL,
--  制作
    make varchar(20) NOT NULL,
--  封面
    logo text NOT NULL,
--  本地标志
    localFlag boolean NOT NULL,
--  本地URL
    localUrl varchar(50),
--  远程标志
    remoteFlag boolean NOT NULL,
--  远程URL
    remoteUrl text,
--  容器格式
    container varchar(10) NOT NULL,
--  视频编码格式
    codev varchar(10) NOT NULL,
--  音频编码格式
    codea varchar(10) NOT NULL,
--  字幕类型
    subType varchar(10) NOT NULL,
--  字幕组
    subTeam varchar(20),
--  最后观看时间
    lastWatch date NOT NULL Default '2012-12-12',
--  更新时间
    updateTime date NOT NULL Default now(),
--  备注
    remark text
);

Insert into Film (
    seriesFlag,
    seriesId,
    Film_name,
    Film_year,
    director,
    screenWriter,
    make,
    logo,
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
    '妇愁者联盟3',
    '2018-04-27',
    '安东尼·罗素、乔·罗素',
    '杰克·科比、克里斯托弗·马库斯、斯蒂芬·麦克菲利、斯坦·李',
    '漫威影业公司',
    'https://static.7os.top/Image/Sevx/APPTest/黎歌Neeko_91500837.jpg',
    True,
    'Disk::M::/Film/妇联3',
    False,
    '',
    'mkv',
    'HEVC',
    'Flac',
    '内挂',
    '不知道',
    ''
);

Insert into Film (
    seriesFlag,
    seriesId,
    Film_name,
    Film_year,
    director,
    screenWriter,
    make,
    logo,
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
    '妇愁者联盟4',
    '2019-04-24',
    '安东尼·罗素、乔·罗素',
    '杰克·科比、克里斯托弗·马库斯、斯蒂芬·麦克菲利、斯坦·李',
    '漫威影业公司',
    'https://static.7os.top/Image/Sevx/APPTest/黎歌Neeko_91500837.jpg',
    True,
    'Disk::M::/Film/妇联4',
    False,
    '',
    'mkv',
    'HEVC',
    'Flac',
    '内挂',
    '不知道',
    '这是一个测试备注'
);


------------------------------------------------------
--    TV 表
------------------------------------------------------
CREATE TABLE TV(
--  id
    id serial NOT NULL PRIMARY KEY,
--  系列Flag
    seriesFlag boolean NOT NULL,
--  系列ID
    seriesId smallint NOT NULL Default 0,
--  电视剧 名称
    Tv_name varchar(50) NOT NULL,
--  电视剧 年份
    Tv_year date NOT NULL,
--  导演
    director varchar(20) NOT NULL,
--  编剧
    screenWriter varchar(20) NOT NULL,
--  出品
    make varchar(20) NOT NULL,
--  封面
    logo text NOT NULL,
--  集数
    amount smallint NOT NULL,
--  本地标志
    localFlag boolean NOT NULL,
--  本地URL
    localUrl varchar(50),
--  远程标志
    remoteFlag boolean NOT NULL,
--  远程URL
    remoteUrl text,
--  容器格式
    container varchar(10) NOT NULL,
--  视频编码格式
    codev varchar(10) NOT NULL,
--  音频编码格式
    codea varchar(10) NOT NULL,
--  字幕类型
    subType varchar(10) NOT NULL,
--  字幕组
    subTeam varchar(20),
--  最后观看时间
    lastWatch date NOT NULL Default '2012-12-12',
--  更新时间
    updateTime date NOT NULL Default now(),
--  备注
    remark text
);

Insert into TV (
    seriesFlag,
    seriesId,
    Tv_name,
    Tv_year,
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
    False,
    0,
    '测试 TV',
    '2018-10-02',
    '导演',
    '编剧',
    '出品方',
    'https://static.7os.top/Image/Sevx/APPTest/黎歌Neeko_91500837.jpg',
    '25',
    True,
    'Disk::M::/哈哈',
    False,
    '',
    'mkv',
    'H264',
    'Flac',
    '无',
    '哈哈哈哈',
    '这是一个测试备注'
);

Insert into TV (
    seriesFlag,
    seriesId,
    Tv_name,
    Tv_year,
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
    False,
    0,
    '测试 TV2',
    '2018-10-02',
    '导演2',
    '编剧2',
    '出品方2',
    'https://static.7os.top/Image/Sevx/APPTest/黎歌Neeko_91500837.jpg',
    '25',
    True,
    'Disk::M::/哈哈2',
    False,
    '',
    'mkv',
    'H264',
    'Flac',
    '无',
    '哈哈哈哈2',
    '这是一个测试备注2'
);


------------------------------------------------------
--    SV 表
------------------------------------------------------
CREATE TABLE SV(
--  ID
    id serial NOT NULL PRIMARY KEY,
--  短片 名称
    SV_name varchar(50) NOT NULL,
--  短片 年份
    SV_year date NOT NULL,
--  短片类型
    SV_type varchar(10) NOT NULL,
--  Logo
    logo text NOT NULL,
--  作者
    author varchar(20) NOT NULL,
--  本地标志
    localFlag boolean NOT NULL,
--  本地URL
    localUrl varchar(50),
--  远程标志
    remoteFlag boolean NOT NULL,
--  远程URL
    remoteUrl text,
--  容器格式
    container varchar(10) NOT NULL,
--  视频编码格式
    codev varchar(10) NOT NULL,
--  音频编码格式
    codea varchar(10) NOT NULL,
--  更新时间
    updateTime date NOT NULL Default now(),
--  备注
    remark text
);

Insert into SV (
    Sv_name,
    Sv_year,
    SV_type,
    logo,
    author,
    localFlag,
    localUrl,
    remoteFlag,
    remoteUrl,
    container,
    codev,
    codea,
    remark
) Values (
    '测试SV',
    '2018-10-02',
    'AMV',
    'https://static.7os.top/Image/Sevx/APPTest/黎歌Neeko_91500837.jpg',
    'Sakura70s',
    True,
    'Disk::M::/哈哈',
    False,
    '',
    'mkv',
    'H264',
    'Flac',
    '这是一个测试备注'
);

Insert into SV (
    Sv_name,
    Sv_year,
    SV_type,
    logo,
    author,
    localFlag,
    localUrl,
    remoteFlag,
    remoteUrl,
    container,
    codev,
    codea,
    remark
) Values (
    '测试SV1',
    '2018-10-02',
    'AMV',
    'https://static.7os.top/Image/Sevx/APPTest/黎歌Neeko_91500837.jpg',
    'Sakura70s',
    True,
    'Disk::M::/哈哈1',
    False,
    '',
    'mkv',
    'H264',
    'Flac',
    '这是一个测试备注1'
);


------------------------------------------------------
--    Music 表
------------------------------------------------------
CREATE TABLE Music(
--  ID
    id serial NOT NULL PRIMARY KEY,
--  音乐 名称
    Music_name varchar(50) NOT NULL,
--  音乐 年份
    Music_year date NOT NULL,
--  专辑封面
    logo text NOT NULL,
--  艺术家
    artist varchar(20) NOT NULL,
--  专辑
    album varchar(20) NOT NULL,
--  作词
    lyrics varchar(20) NOT NULL,
--  作曲
    written varchar(20) NOT NULL,
--  本地Flag
    localFlag boolean NOT NULL,
--  本地URL
    localUrl varchar(50),
--  远程Flag
    remoteFlag boolean NOT NULL,
--  远程URL
    remoteUrl text,
--  容器格式
    container varchar(10) NOT NULL,
--  歌词类型
    lyricType varchar(10) NOT NULL,
--  更新时间
    updateTime date NOT NULL Default now(),
--  备注
    remark text
);

Insert into Music(
    Music_name,
    Music_year,
    logo,
    artist,
    album,
    lyrics,
    written,
    localFlag,
    localUrl,
    remoteFlag,
    remoteUrl,
    container,
    lyricType,
    remark
) Values (
    '稻香',
    '2008-10-15',
    'https://static.7os.top/Image/Sevx/APPTest/6250c051ba9462ce.jpg',
    '周杰伦',
    '魔杰座',
    '周杰伦',
    '周杰伦',
    True,
    'C::Music::',
    True,
    'https://cloud.7os.top/Music/稻香.flac',
    'Flac',
    '内挂',
    '无'
);

Insert into Music(
    Music_name,
    Music_year,
    logo,
    artist,
    album,
    lyrics,
    written,
    localFlag,
    localUrl,
    remoteFlag,
    remoteUrl,
    container,
    lyricType,
    remark
) Values (
    '稻香1',
    '2008-10-15',
    'https://static.7os.top/Image/Sevx/APPTest/6250c051ba9462ce.jpg',
    '周杰伦',
    '魔杰座',
    '周杰伦',
    '周杰伦',
    True,
    'C::Music::',
    True,
    'https://cloud.7os.top/Music/稻香.flac',
    'Flac',
    '内挂',
    '111111111'
);


------------------------------------------------------
--    Novel 表
------------------------------------------------------
CREATE TABLE Novel(
--  ID
    id serial NOT NULL PRIMARY KEY,
--  系列Flag
    seriesFlag boolean NOT NULL,
--  系列ID
    seriesId smallint NOT NULL Default 0,
--  小说 名称
    Novel_name varchar(50) NOT NULL,
--  小说 年份
    Novel_year date NOT NULL,
--  连载状态
    Novel_status varchar(20) NOT NULL,
--  封面
    logo text NOT NULL,
--  作者
    author varchar(20) NOT NULL,
--  本地标志
    localFlag boolean NOT NULL,
--  本地URL
    localUrl varchar(50),
--  远程标志
    remoteFlag boolean NOT NULL,
--  远程URL
    remoteUrl text,
--  容器格式
    container varchar(10) NOT NULL,
--  更新时间
    updateTime date NOT NULL Default now(),
--  备注
    remark text
);

Insert into Novel(
    seriesFlag,
    seriesId,
    Novel_name,
    Novel_year,
    Novel_status,
    logo,
    author,
    localFlag,
    localUrl,
    remoteFlag,
    remoteUrl,
    container,
    remark
) Values (
    False,
    0,
    'Test Novel',
    '2000-01-01',
    '连载',
    'https://static.7os.top/Image/Sevx/APPTest/E1912F9D9FEB9F292823F27F5B516D3A.jpg',
    'Sakura70s',
    False,
    '',
    False,
    '',
    'Epub',
    'Mark'
);

Insert into Novel(
    seriesFlag,
    seriesId,
    Novel_name,
    Novel_year,
    Novel_status,
    logo,
    author,
    localFlag,
    localUrl,
    remoteFlag,
    remoteUrl,
    container,
    remark
) Values (
    False,
    0,
    'Test Novel1',
    '2000-01-01',
    '连载',
    'https://static.7os.top/Image/Sevx/APPTest/E1912F9D9FEB9F292823F27F5B516D3A.jpg',
    'Sakura70s',
    False,
    '',
    False,
    '',
    'Epub',
    'Mark1'
);

------------------------------------------------------
--    Comic 表
------------------------------------------------------
CREATE TABLE Comic(
--  id
    id serial NOT NULL PRIMARY KEY,
--  系列Flag
    seriesFlag boolean NOT NULL,
--  系列ID
    seriesId smallint NOT NULL Default 0,
--  漫画 名称
    Comic_name varchar(50) NOT NULL,
--  漫画 年份
    Comic_year date NOT NULL,
--  连载状态
    Comic_status varchar(20) NOT NULL,
--  封面
    logo text NOT NULL,
--  作者
    author varchar(20) NOT NULL,
--  本地标志
    localFlag boolean NOT NULL,
--  本地URL
    localUrl varchar(50),
--  远程标志
    remoteFlag boolean NOT NULL,
--  远程URL
    remoteUrl text,
--  容器格式
    container varchar(10) NOT NULL,
--  更新时间
    updateTime date NOT NULL Default now(),
--  备注
    remark text
);

Insert into Comic(
    seriesFlag,
    seriesId,
    Comic_name,
    Comic_year,
    Comic_status,
    logo,
    author,
    localFlag,
    localUrl,
    remoteFlag,
    remoteUrl,
    container,
    remark
) Values (
    False,
    0,
    '我是娇小魅魔的忠实仆从',
    '2018-01-01',
    '完结',
    'https://static.7os.top/Image/Sevx/APPTest/243659b910rvsQr.jpg',
    '玉之けだま',
    False,
    '',
    False,
    '',
    'Zip',
    '测试备注1'
);

Insert into Comic(
    seriesFlag,
    seriesId,
    Comic_name,
    Comic_year,
    Comic_status,
    logo,
    author,
    localFlag,
    localUrl,
    remoteFlag,
    remoteUrl,
    container,
    remark
) Values (
    False,
    0,
    '我是娇小魅魔的忠实仆人1',
    '2018-01-01',
    '完结',
    'https://static.7os.top/Image/Sevx/APPTest/243659b910rvsQr.jpg',
    '玉之けだま',
    False,
    '',
    False,
    '',
    'Zip',
    '测试备注2'
);
