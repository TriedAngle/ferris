CREATE TABLE guild_infos
(
    guild_id BIGINT NOT NULL,
    PRIMARY KEY (guild_id)
);

CREATE TABLE reaction_roles
(
    message_id BIGINT                NOT NULL,
    guild_id   BIGINT                NOT NULL,
    channel_id BIGINT                NOT NULL,
    emoji      CHARACTER VARYING(25) NOT NULL,
    role_id    BIGINT                NOT NULL,
    animated   BOOLEAN,
    emoji_name TEXT,
    PRIMARY KEY (message_id, emoji),
    CONSTRAINT "fk_reaction_roles_guild_info" FOREIGN KEY (guild_id)
        REFERENCES guild_infos (guild_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
);

CREATE TABLE users
(
    id              SERIAL      NOT NULL,
    discord_id      TEXT        NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE shess_games
(
    id              UUID        NOT NULL,
    channel_id      TEXT        NOT NULL,
    players         INT[]       NOT NULL,
    running         BOOLEAN     NOT NULL,
    PRIMARY KEY (id)
)