# Build stage
FROM maven:3.9-eclipse-temurin-17 AS build
WORKDIR /build
COPY pom.xml .
COPY src ./src
RUN mvn -B package -DskipTests -Dquarkus.package.jar.type=uber-jar

# Runtime — isolated solver compute node
FROM eclipse-temurin:17-jre-alpine
WORKDIR /app
RUN addgroup -S solver && adduser -S solver -G solver
USER solver:solver
COPY --from=build /build/target/*-runner.jar app.jar

ENV JAVA_OPTS="-Xms256m -Xmx768m -XX:+UseG1GC"
EXPOSE 8081
ENTRYPOINT ["sh", "-c", "java $JAVA_OPTS -jar app.jar"]
