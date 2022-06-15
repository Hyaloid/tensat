docker run --gpus all --pid=host --net=host \
--name try \
-it \
--mount type=bind,source="/home/zhaohs/tensat_test",target=/usr/tensat \
--mount type=bind,source="/home/zhaohs/egg",target=/usr/egg \
--mount type=bind,source="/home/zhaohs/TASO",target=/usr/TASO \
tensat_test bash
