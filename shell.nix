{ pkgs }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    sqlx-cli
    cargo-watch
    git-cliff
    cargo-release
    cargo-sort
    cargo-deny
    postgresql
  ];

  shellHook = ''
    export RUST_BACKTRACE=1
    export RUST_LOG=debug
    PGDATA=$PWD/.pgdata
    PGDATABASE=postgres
    PGUSER=postgres
    PGPASSWORD=password
    PGPORT=5432
    PGHOST=localhost

    # Database Configuration
    export DATABASE_URL=postgres://$PGUSER:$PGPASSWORD@$PGHOST:$PGPORT/$PGDATABASE
    export APP__DATABASE__USERNAME=$PGUSER
    export APP__DATABASE__PASSWORD=$PGPASSWORD
    export APP__DATABASE__HOST=$PGHOST
    export APP__DATABASE__PORT=$PGPORT
    export APP__DATABASE__NAME=$PGDATABASE
    export APP__DATABASE__MAX_CONNECTIONS=10
    export APP__DATABASE__MIN_CONNECTIONS=1
    export APP__DATABASE__ACQUIRE_TIMEOUT_SECS=5

    # Server Configuration
    export APP__SERVER__HOST=127.0.0.1
    export APP__SERVER__PORT=8080
    export APP__SERVER__TIMEOUT_IN_SECS=10
    export APP__SERVER__ORIGINS=http://localhost:3000
    export APP__SERVER__RATE_LIMIT_PER_SECS=100
    export APP__SERVER__RATE_LIMIT_BURST=10
    export APP__SERVER__COOKIE_SECRET=$(openssl rand -base64 64)

    # Environment Configuration
    export APP__ENVIRONMENT=local

    # JWT Configuration
    export APP__JWT__SECRET=$(openssl rand -base64 64)
    export APP__JWT__ACCESS_TOKEN_EXPIRATION_SECS=3600
    export APP__JWT__REFRESH_TOKEN_EXPIRATION_SECS=86400

    if [ ! -d "$PGDATA" ]; then
      initdb -D $PGDATA --username=$PGUSER --pwfile=<(echo $PGPASSWORD)
      echo "host all all 127.0.0.1/32 md5" >> $PGDATA/pg_hba.conf
    fi

    # Check if PostgreSQL is already running
    if ! pg_ctl status -D $PGDATA > /dev/null; then
      pg_ctl -D $PGDATA -o "-p $PGPORT" -l logfile start
    fi

    # Define cleanup function
    function end {
      echo "Cleaning up PostgreSQL server..."
      pg_ctl -D "$PGDATA" stop
    }

    # Cleanup PostgreSQL server on exit
    trap end EXIT

    # Run migrations
    sqlx migrate run

    # Set the default shell to zsh
    export SHELL=$(which zsh)
    exec zsh
  '';
}
