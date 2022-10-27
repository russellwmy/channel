#!/bin/bash
docker run -u 1000:1000 -p 8080:8080 --rm -it -v $PWD:/pwd channel-dev-env /bin/bash