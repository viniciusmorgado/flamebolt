# flamebolt

Flamebolt is a self-hosted serverless platform built on top of the Amazon Firecracker microVM engine. It enables deployment and execution of web applications in lightweight microVMs, providing an experience similar to AWS ECS and AWS Lambda — but running on your own Linux server infrastructure using Firecracker.

Known Limitations

- Scale-to-Zero is only supported in stateless containers and functions.
