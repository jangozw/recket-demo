# rocket-demo
http api, response, db operations.
## run 

```shell
cargo run
```
print: 
```txt
📬 Routes:
   >> (user_list) GET /v1/user/list
   >> (user_detail) GET /v1/user/<uid>
   >> (order_list) GET /v1/order/list
📡 Fairings:
   >> Shield (liftoff, response, singleton)
🛡️ Shield:
   >> X-Frame-Options: SAMEORIGIN
   >> X-Content-Type-Options: nosniff
   >> Permissions-Policy: interest-cohort=()
🚀 Rocket has launched from http://127.0.0.1:8001
```
now you can open url in your browser!


## base data

msyql connetction: 
```text
database: test
user: root
pass: 123456
ip: localhost
```
execute sql:
```mysql
create database test;
use test;
CREATE TABLE IF NOT EXISTS `user` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `age` int NOT NULL DEFAULT '0',
  `info` text COLLATE utf8mb4_unicode_ci,
  `created_at` datetime DEFAULT NULL,
  `updated_at` datetime DEFAULT CURRENT_TIMESTAMP,
  `deleted_at` datetime DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

INSERT INTO `user` VALUES (3, '李云龙', 33, '师长', '2022-06-29 08:59:55', '2022-06-29 08:59:55', NULL);
INSERT INTO `user` VALUES (4, '赵云', 45, '团长', '2022-06-23 08:59:55', '2022-06-29 09:00:40', NULL);
INSERT INTO `user` VALUES (5, '刘强东', 42, '京东', NULL, '2022-06-29 09:01:36', NULL);
INSERT INTO `user` VALUES (6, '李文艺', 22, '上海市长', NULL, '2022-06-29 10:25:53', NULL);

```
test
