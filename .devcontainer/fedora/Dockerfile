FROM fedora:40

ARG USERNAME=user
ARG USER_UID=1000
ARG USER_GID=$USER_UID
ARG ARCH
ENV TZ="America/New_York"

# Install packages
RUN dnf update -y \
    && dnf install -y curl net-tools git vim unzip which sudo tmux dos2unix \
        gcc openssl-devel bzip2-devel libffi-devel zlib-devel wget make \
    && dnf group install -y "C Development Tools and Libraries" "Development Tools" \
    && dnf -y clean all
    
# Add a non-root user and enable them to use sudo without password
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd -s /bin/bash --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && sed -i /etc/sudoers -re 's/^%sudo.*/%sudo ALL=(ALL:ALL) NOPASSWD: ALL/g' \
    && sed -i /etc/sudoers -re 's/^root.*/root ALL=(ALL:ALL) NOPASSWD: ALL/g' \
    && sed -i /etc/sudoers -re 's/^#includedir.*/## **Removed the include directive** ##"/g' \
    && echo "$USERNAME ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers \
    && echo "Customized the sudoers file for passwordless access to the $USERNAME user!" \
    && echo "$USERNAME user:";  su - $USERNAME -c id

USER $USERNAME
WORKDIR /home/$USERNAME/

# add docker
# From <https://docs.docker.com/engine/install/fedora/>
# From <https://github.com/microsoft/vscode-dev-containers/tree/main/containers/docker-from-docker-compose>
# From <https://github.com/microsoft/vscode-dev-containers/tree/master/containers/docker-from-docker>
RUN sudo dnf config-manager --add-repo https://download.docker.com/linux/fedora/docker-ce.repo \
    # step 1 enabling root access - install docker cli  (don't forget to add mount in devcontainer.json)
    && sudo dnf install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin socat \
    && sudo usermod -aG docker $USERNAME \
    # step 2 enabling non-root access
    # Default to root only access to the Docker socket, set up non-root init script
    && sudo touch /var/run/docker-host.sock \
    && sudo ln -s /var/run/docker-host.sock /var/run/docker.sock \
    # Create docker-init.sh to spin up socat
    && echo -e "#!/bin/sh\n\
sudoIf() { if [ \"\$(id -u)\" -ne 0 ]; then sudo \"\$@\"; else \"\$@\"; fi }\n\
sudoIf rm -rf /var/run/docker.sock\n\
((sudoIf socat UNIX-LISTEN:/var/run/docker.sock,fork,mode=660,user=${USERNAME} UNIX-CONNECT:/var/run/docker-host.sock) 2>&1 >> /tmp/vscr-docker-from-docker.log) & > /dev/null\n\
\"\$@\"" | sudo tee -a /usr/local/share/docker-init.sh \
    && sudo chmod +x /usr/local/share/docker-init.sh \
    # Add non-root user to docker group
    && sudo usermod -aG docker $USERNAME

# install rust
ENV CARGO_HOME="/home/$USERNAME/.cargo"
ENV RUSTUP_HOME="/home/$USERNAME/.rustup"
ENV PATH="$CARGO_HOME/bin:$RUSTUP_HOME:$PATH"

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -sSf | sh -s -- --verbose -y --profile default \
    && rustup component add rust-analysis \
    && rustup component add rust-src \
    && rustup component add rls \
    && rustup target add x86_64-unknown-linux-gnu \
    && rustup target add x86_64-unknown-linux-musl \
    && rustup target add x86_64-pc-windows-msvc \
    && rustup target add aarch64-unknown-linux-gnu \
    && rustup target add aarch64-pc-windows-msvc \
    && rustup target add aarch64-apple-darwin \
    && rustup target add wasm32-wasi

# install starship and terminal configuration
RUN curl -sS https://starship.rs/install.sh | sh -s -- --verbose -y \
    && echo "eval '$(starship init bash)'" >> /home/$USERNAME/.bashrc \
    && echo "eval '$(starship init zsh)'" >> /home/$USERNAME/.zshrc

ADD ./starship_config/starship.toml /home/$USERNAME/.config/starship.toml

# Setting the ENTRYPOINT to docker-init.sh will configure non-root access 
# to the Docker socket. The script will also execute CMD as needed.
ENTRYPOINT [ "/usr/local/share/docker-init.sh" ] 
CMD [ "sleep", "infinity" ]