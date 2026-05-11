a. What is amqp?
AMQP (Advanced Message Queuing Protocol) is an open standard protocol for message-oriented middleware. It defines how messages are formatted, routed, and transferred between applications through a message broker. In this tutorial, RabbitMQ acts as the broker and communicates using AMQP.

b. What does it mean? guest:guest@localhost:5672 , what is the first guest, and what is the second guest, and what is localhost:5672 is for?

First guest, the username used to authenticate with the RabbitMQ broker.

Second guest, the password for that username. guest is a RabbitMQ's default credential.

localhost:5672, the host and port where RabbitMQ is running. localhost means it's running on your own machine, and 5672 is the default port for AMQP connections.


a. How much data will the publisher send to the message broker in one run?
The publisher sends 5 messages in one run, one for each user. Each message is a UserCreatedEventMessage struct containing two string fields: user_id and user_name. The data is serialized using Borsh before being sent, so the actual byte size depends on the string lengths, but logically it is 5 events total per run.

b. The URL amqp://guest:guest@localhost:5672 is the same in both publisher and subscriber, what does it mean?
It means both the publisher and subscriber are connecting to the same RabbitMQ broker instance. The publisher sends messages to the broker, and the subscriber listens and consumes messages from the broker. They don't communicate directly with each other, the broker is the middleman. This is the core idea of event-driven architecture: the publisher and subscriber are decoupled and only need to know the broker's address, not each other's.

![alt text](<Screenshot 2026-05-11 204712-1.png>)

![alt text](<Screenshot 2026-05-11 212118.png>)