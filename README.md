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

