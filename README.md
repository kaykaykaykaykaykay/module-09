a. What is amqp?
AMQP (Advanced Message Queuing Protocol) is an open standard protocol for message-oriented middleware. It defines how messages are formatted, routed, and transferred between applications through a message broker. In this tutorial, RabbitMQ acts as the broker and communicates using AMQP.

b. What does it mean? guest:guest@localhost:5672 , what is the first guest, and what is the second guest, and what is localhost:5672 is for?

First guest, the username used to authenticate with the RabbitMQ broker.

Second guest, the password for that username. guest is a RabbitMQ's default credential.

localhost:5672, the host and port where RabbitMQ is running. localhost means it's running on your own machine, and 5672 is the default port for AMQP connections.