## Subscriber
a. What is amqp?
AMQP (Advanced Message Queuing Protocol) is an open standard protocol for message-oriented middleware. It defines how messages are formatted, routed, and transferred between applications through a message broker. In this tutorial, RabbitMQ acts as the broker and communicates using AMQP.

b. What does it mean? guest:guest@localhost:5672 , what is the first guest, and what is the second guest, and what is localhost:5672 is for?

First guest, the username used to authenticate with the RabbitMQ broker.

Second guest, the password for that username. guest is a RabbitMQ's default credential.

localhost:5672, the host and port where RabbitMQ is running. localhost means it's running on your own machine, and 5672 is the default port for AMQP connections.

## Publisher
a. How much data will the publisher send to the message broker in one run?
The publisher sends 5 messages in one run, one for each user. Each message is a UserCreatedEventMessage struct containing two string fields: user_id and user_name. The data is serialized using Borsh before being sent, so the actual byte size depends on the string lengths, but logically it is 5 events total per run.

b. The URL amqp://guest:guest@localhost:5672 is the same in both publisher and subscriber, what does it mean?
It means both the publisher and subscriber are connecting to the same RabbitMQ broker instance. The publisher sends messages to the broker, and the subscriber listens and consumes messages from the broker. They don't communicate directly with each other, the broker is the middleman. This is the core idea of event-driven architecture: the publisher and subscriber are decoupled and only need to know the broker's address, not each other's.

![alt text](<Screenshot 2026-05-11 204712-1.png>)

![alt text](<Screenshot 2026-05-11 214145.png>)

## Spike in RabbitMQ Chart

The spike in the RabbitMQ management console chart is directly caused by running the publisher. Each time the publisher runs, it pushes all 5 messages into the `user_created` queue almost simultaneously in a single burst. The subscriber, however, processes messages one at a time and sequentially — so for a brief moment, messages arrive faster than they are consumed, causing the queue depth to spike upward. As the subscriber catches up, the queue drains and the chart falls back down.

Running the publisher multiple times in quick succession amplifies this effect. For example, 5 runs produce 25 messages entering the queue nearly all at once, making the spike more pronounced and the drain period longer.

This demonstrates the core value of using a message broker. The publisher is completely decoupled from the subscriber's processing speed — it fires and forgets without waiting. The broker acts as a buffer, absorbing the burst of messages and delivering them to the subscriber at whatever pace it can handle. Without the broker, the publisher would either have to wait for each message to be processed (tight coupling) or messages would simply be dropped.

## Slow Subscriber Simulation

![alt text](<Screenshot 2026-05-11 215431.png>)

Looking at the top graph (Queued messages), you can see multiple spikes across the last minute, each spike represents one publisher run pushing 5 messages into the queue at once. The publisher was run multiple times in quick succession, producing a total of 20 messages across 4 runs (4 × 5 = 20).

The current Total: 0 (Ready: 0, Unacked: 0) means the queue is now completely empty. Even though 20 messages were queued up, the subscriber processed all of them one by one. With `thread::sleep` active, the subscriber handled roughly 1 message per second, so the queue drained gradually after each burst. By the time this screenshot was taken, the subscriber had caught up and nothing remained in the queue.

This is the point of using a message broker, the publisher can fire off all 20 messages instantly without caring whether the subscriber is ready, and the broker safely holds them until the subscriber processes every single one.


## Running 3 Subscribers

![alt text](<Screenshot 2026-05-12 111621.png>)

With 3 subscribers running simultaneously, the RabbitMQ chart looks completely different. The queued messages graph stays essentially flat at 0 throughout, and the Global counts show **Connections: 3, Channels: 3, Consumers: 3**. This is because RabbitMQ distributes messages across all active consumers in a round-robin fashion. When the publisher sends 5 messages, RabbitMQ splits them across the 3 subscribers — roughly 2 each with 1 left over. Each subscriber still has `thread::sleep` (1 second per message), but together they process ~3 messages per second instead of 1. The queue drains fast enough that no visible backlog builds up. This is the competing consumers pattern: adding more subscribers directly increases throughput without changing any publisher code.

## Code Improvements

Looking at the publisher and subscriber code, there are a few things worth improving:

**Publisher**
- The AMQP URL is hard-coded as `"amqp://guest:guest@localhost:5672"`. The subscriber already reads from the `AMQP_URL` environment variable with a fallback, the publisher should do the same for consistency and so both can be configured from a single place.
- All five `p.send(...)` calls use `_ =` to discard the result, silently swallowing any send errors. Failed sends should at least be logged.

**Subscriber**
- The variable is named `ten_millis` but is set to `1000` milliseconds (1 full second), not 10ms. This is a misleading name that could confuse anyone reading the code.
- `loop {}` at the end of `main` is a busy-wait, it spins the thread at full CPU doing nothing. A better alternative is `std::thread::park()`, which blocks the thread without consuming CPU.
- Like the publisher, `_ = subscriber.subscribe(...)` silently discards the subscription error if it fails.