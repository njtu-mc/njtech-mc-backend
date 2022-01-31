FROM ubuntu:impish

WORKDIR /app
COPY target/release/njtech-mc-backend njtech-mc-backend

EXPOSE 8080

CMD [ "/app/njtech-mc-backend" ]
