Just random ideas as they flow in my brain.

## Ideas

```
# load_dotenv = true
# root = ../www
# environment = { NODE_ENV = "production" }

#[[init]]
#cmd = ""

#[[test]]
#cmd = ""

#[[dev]]
#cmd = ""

[[run]]
cmd = "ls"
#watch = true

[[run]]
cmd = ["ls"]
#once = true
#tags = ["test"]
#after = ["tag1"]
#before = ["tag2"]
```