FROM alpine:latest AS stage1
WORKDIR /workdir1
RUN sleep 5
RUN echo 'stage1' >> stage1.txt

FROM alpine:latest AS stage2
WORKDIR /workdir2
RUN sleep 5
COPY --from=stage1 /workdir1/stage1.txt /workdir2/stage1.txt
RUN echo 'stage2' >> stage2.txt

FROM alpine:latest AS stage3
WORKDIR /workdir
COPY --from=stage1 /workdir1/stage1.txt /workdir/stage1.txt
COPY --from=stage2 /workdir2/stage1.txt /workdir/stage1-2.txt
COPY --from=stage2 /workdir2/stage2.txt /workdir/stage2.txt
