# Instructions for use with Docker:

Minor heads up: Rust's cargo build takes quite a bit of time and if not given
enough resources it'll crash near the end of the build just to spite you. If you
get some "error: cc" message then you need to allow docker to use more memory.
This will not be an issue once the image is published on dockerhub since we'll
be skipping the build at that point.

Similarly this also means you have to reference the build in the docker-compose
file for now.

**Make sure you're using the VPN when running a node otherwise you'll find no
peers!**

<hr/>

### What you need to care about

There's only 2 arguments that we need to pay attention to when booting up a new
node (all ports are set to defaults to make life easier with preset security
groups), and those are:

- bootnodeIP = xxx.xxx.xxx.xxx
- nodename = testNode000

The former is important as it is the IP of the very first myChain instance to
start the chain. This can be found by checking the public IP of "Node-MyChain"
on AWS.

The latter is just an identifier for the node.

The stealth parameter you'll have to watch out for is the assignment of a volume
to `/data` on the container. You will need a link defined on your docker run
command / docker-compose file so you don't have to deal with looking through
hidden folders for the docker storage later. You can check the sample
docker-compose file for more.

<hr/>

## TL;DR

Ignore the rest of this repo, install docker on your machine of choice, copy
this folder and run `docker-compose up -d` in it.
