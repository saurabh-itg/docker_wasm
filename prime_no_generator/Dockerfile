# Use a base image with the desired Linux distribution
FROM ubuntu:latest

# Install necessary dependencies
RUN apt-get update && apt-get install -y curl

# Download and install Wasmer
RUN curl https://get.wasmer.io -sSfL | sh

# Add Wasmer binaries to the system PATH
ENV PATH="/root/.wasmer/bin:${PATH}"


# Print Wasmer version
# CMD ["wasmer", "--version"]

# Copy project file
COPY ./* ./

# Run the program
CMD ["wasmer", "run", "prime_no_generator.wasm"]
