FROM openjdk:19-jdk-alpine
RUN mkdir /opt/applications
COPY target/strength-0.0.1-SNAPSHOT.jar /opt/applications/
EXPOSE 8080
CMD [ "java", "-jar", "/opt/applications/strength-0.0.1-SNAPSHOT.jar" ]