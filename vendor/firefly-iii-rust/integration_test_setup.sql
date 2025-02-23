USE firefly;
INSERT INTO user_groups (id, created_at, updated_at, deleted_at, title)
VALUES (1, NOW(), NOW(), NOW(), 'test');
INSERT INTO users ( id, created_at, updated_at, email, password, blocked, user_group_id)
VALUES (1, NOW(), NOW(), 'example@example.com', '$2y$10$QJ/0n1Ag/KBSYVbggxnSVeOHoO5PhnPs5mx5LwJkMIaDKG0X4A/gW', 0, 1);
