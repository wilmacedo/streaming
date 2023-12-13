# Use the official Gradle image with JDK 8
FROM gradle:8.4-jdk8 AS builder

# Copy the project files into the container
COPY --chown=gradle:gradle . /home/gradle/src

# Set the working directory
WORKDIR /home/gradle/src

# Build the project
RUN gradle build --no-daemon

# Use the official OpenJDK image
FROM openjdk:8-jdk-slim

# Create a directory for the application
RUN mkdir /app

# Set the working directory
WORKDIR /app

# Copy the JAR file from the builder stage to the /app directory
COPY --from=builder /home/gradle/src/build/libs/kt.jar .
COPY --from=builder /home/gradle/src/build/libs/*.jar .

# # Add Kotlin runtime dependencies
# RUN apt-get update && apt-get install -y curl && \
#     curl -s https://get.sdkman.io | bash && \
#     bash -c "source /root/.sdkman/bin/sdkman-init.sh && sdk install kotlin"

# Set the entry point for the container
ENTRYPOINT ["java", "-jar", "/app/kt.jar"]
