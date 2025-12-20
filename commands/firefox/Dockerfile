FROM debian:bookworm-slim

# Install Firefox ESR and required dependencies
RUN apt-get update && \
    apt-get install -y \
    firefox-esr \
    libgtk-3-0 \
    libdbus-glib-1-2 \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user to run Firefox
RUN useradd -m -s /bin/bash firefox

# Switch to non-root user
USER firefox
WORKDIR /home/firefox

# Execute Firefox ESR
ENTRYPOINT ["firefox-esr"]
CMD []
