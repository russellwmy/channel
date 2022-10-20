#!/bin/bash
docker run -u 1000:1000 -p 1234:1234 --rm -it -v $PWD:/pwd channel-dev-env /bin/bash