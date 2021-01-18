CREATE TABLE guild_info
(
    guild_id BIGINT NOT NULL,
    prefix text,
    PRIMARY KEY (guild_id)
);

-- Add migration script here
-- Add migration script here
CREATE TABLE reaction_roles
(
    message_id BIGINT                NOT NULL,
    guild_id   BIGINT                NOT NULL,
    channel_id BIGINT                NOT NULL,
    emoji      CHARACTER VARYING(25) NOT NULL,
    role_id    BIGINT                NOT NULL,
    animated   BOOLEAN,
    emoji_name TEXT,
    PRIMARY KEY (message_id, emoji, role_id),
    CONSTRAINT "fk_reaction_roles_guild_info_guild_id" FOREIGN KEY (guild_id)
        REFERENCES guild_info (guild_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID
)