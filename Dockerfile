# Use a Linux base image
FROM ubuntu:latest

# Update the package lists
RUN apt-get update

# Install curl
RUN apt-get install -y curl
