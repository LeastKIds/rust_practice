FROM rust:1.61.0-slim-buster

MAINTAINER "leastkids@gmail.com"

ENV PYTHONUNBUFFERED 1

RUN mkdir /app
WORKDIR /app
COPY ./app /app
RUN mkdir /hyperlink

