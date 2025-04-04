# Sake - a new Minecraft Bedrock add-on compiler.

Sake supports *actions*, a system that allows programmatic manipulations on packs. 
If you write your actions in Rust, take a look at Sake as a lib - it has useful features for that.

> **THE PROJECT IS STILL BEING WORKED ON AND BREAKING CHANGES MIGHT HAPPEN**

## User setup
NOT YET
## Dev setup
If you want to contribute to Sake, follow this steps
* Clone the repo
* Create a .env file, with the content
```dotenv
SAKE_DEV=true
# optional, so if Sake crashes, you don't have to clean the lock manually
SAKE_DISABLE_LOCK=true
```
* After that, the work directory is now ./tests, use it as if you are a user of Sake