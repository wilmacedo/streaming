FROM gradle:8.5.0-jdk8 AS builder

COPY --chown=gradle:gradle . /home/gradle/src

WORKDIR /home/gradle/src

RUN gradle build --no-daemon

FROM openjdk:8-jre-slim

RUN mkdir /app

COPY --from=builder /home/gradle/src/build/libs/ /app/

ENTRYPOINT ["java","-jar","/app/kotlin-docker-gradle-app.jar"]