CREATE TABLE public.guild_info
(
    guild_id BIGINT NOT NULL,
    PRIMARY KEY (guild_id)
);

CREATE TABLE public.reaction_roles
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
        REFERENCES guild_info (guild_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
)