<h1 align="center">
  Crab notifications 🦀
</h1>

<p align="center">
  <a href="#-technologies">Technologies</a>&nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;
  <a href="#-getting-started">Getting started</a>&nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;
  <a href="#-license">License</a>
</p>

## 👨🏻‍💻 About the project

- <p>Crab Notifications is a notification service written in Rust, designed for educational purposes. It uses RabbitMQ to manage and separate email, push, and SMS notifications. This project showcases the integration of various technologies such as Tokio, Axum, and Amqprs, providing a comprehensive example of building a scalable and maintainable notification system.</p>


## 🚀 Technologies

Technologies that I used to develop this project

- [Tokio](https://tokio.rs/)
- [Axum](https://github.com/tokio-rs/axum)
- [Amqprs](https://github.com/gftea/amqprs)
- [Tracing](https://github.com/tokio-rs/tracing)
- [Resend](https://resend.com/)
- [Firebase cloud messaging](https://firebase.google.com/products/cloud-messaging?hl=pt-br)
- [Twilio](https://www.twilio.com/)

## 💻 Getting started

### Requirements

**Clone the project and access the folder**

```bash
$ git clone https://github.com/joao-garcia404/crab-notifications && cd crab-notifications
```

**Follow the steps below**

```bash
# Setting the env's
$ set -o allexport; source ./src/.env ; set +o allexport

# Running the project
$ cargo run
```

## 📝 License

This project is licensed under the MIT License.

---

Made with 💜 &nbsp;by João Vitor Garcia 👋 &nbsp;[See my linkedin](https://www.linkedin.com/in/joao-garcia404/)
